//! The TCP transport module handles receiving and sending of binary data in chunks, handshake,
//! session creation and dispatching of messages via message handler.
//!

use std;
use std::collections::VecDeque;
use std::io::{Cursor, Write};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{self};

use opcua_core::prelude::*;
use opcua_types::status_codes::StatusCode;
use opcua_types::status_codes::StatusCode::*;

use chrono;
use chrono::Utc;
use time;
use timer;
use futures::Future;
use futures::future::{loop_fn, Loop};
use tokio;
use tokio::net::TcpStream;
use tokio_io::AsyncRead;
use tokio_io::io;
use tokio_io::io::{ReadHalf, WriteHalf};

use address_space::types::AddressSpace;
use comms::secure_channel_service::SecureChannelService;
use constants;
use state::ServerState;
use services::message_handler::MessageHandler;
use session::Session;
use subscriptions::PublishResponseEntry;
use subscriptions::subscription::TickReason;

// TODO these need to go, and use session settings
const RECEIVE_BUFFER_SIZE: usize = 1024 * 64;
const SEND_BUFFER_SIZE: usize = 1024 * 64;
const MAX_MESSAGE_SIZE: usize = 1024 * 64;

#[derive(Clone, Debug, PartialEq)]
pub enum TransportState {
    New,
    WaitingHello,
    ProcessMessages,
    Finished,
}

/// Subscription events are passed between the timer thread and the session thread so must
/// be transferable
#[derive(Clone, Debug, PartialEq)]
enum SubscriptionEvent {
    PublishResponses(VecDeque<PublishResponseEntry>),
}

pub trait Transport {
    // Get the current state of the transport
    fn state(&self) -> TransportState;
    /// Gets the session status
    fn session_status(&self) -> StatusCode;
    /// Sets the session status
    fn set_session_status(&mut self, session_status: StatusCode);
    /// Gets the session associated with the transport
    fn session(&self) -> Arc<RwLock<Session>>;
    /// Returns the address of the client (peer) of this connection
    fn client_address(&self) -> Option<SocketAddr>;
}

/// This is the thing that handles input and output for the open connection associated with the
/// session.
pub struct TcpTransport {
    // Server state, address space etc.
    server_state: Arc<RwLock<ServerState>>,
    // Session status code - Good, BadInvalid etc.
    session_status: StatusCode,
    // Session state - open sessions, tokens etc
    session: Arc<RwLock<Session>>,
    /// Address space
    address_space: Arc<RwLock<AddressSpace>>,
    /// The current transport state
    transport_state: TransportState,
    /// Client address
    client_address: Option<SocketAddr>,
    /// Secure channel handler
    secure_channel_service: SecureChannelService,
    /// Message handler
    message_handler: MessageHandler,
    /// Client protocol version set during HELLO
    client_protocol_version: UInt32,
    /// Last encoded sequence number
    last_sent_sequence_number: UInt32,
    /// Last decoded sequence number
    last_received_sequence_number: UInt32,
    /// Subscription timer
    subscription_events: VecDeque<SubscriptionEvent>,
}

struct ConnectionState {
    pub connection: Arc<RwLock<TcpTransport>>,
    pub read_buffer: MessageBuffer,
    pub send_buffer: Cursor<Vec<u8>>,
    pub reader: ReadHalf<TcpStream>,
    pub writer: WriteHalf<TcpStream>,
    pub in_buf: Vec<u8>,
    pub bytes_read: usize,
    pub hello_timeout: chrono::Duration,
    pub session_start_time: chrono::DateTime<Utc>,
}

impl Transport for TcpTransport {
    fn state(&self) -> TransportState {
        self.transport_state.clone()
    }

    fn session_status(&self) -> StatusCode {
        self.session_status
    }

    fn set_session_status(&mut self, session_status: StatusCode) {
        self.session_status = session_status
    }

    fn session(&self) -> Arc<RwLock<Session>> {
        self.session.clone()
    }

    fn client_address(&self) -> Option<SocketAddr> {
        self.client_address
    }
}

impl TcpTransport {
    pub fn new(server_state: Arc<RwLock<ServerState>>, session: Arc<RwLock<Session>>, address_space: Arc<RwLock<AddressSpace>>, message_handler: MessageHandler) -> TcpTransport {
        let secure_channel_service = SecureChannelService::new();
        TcpTransport {
            server_state,
            session,
            session_status: Good,
            address_space,
            transport_state: TransportState::New,
            client_address: None,
            message_handler,
            secure_channel_service,
            client_protocol_version: 0,
            last_sent_sequence_number: 0,
            last_received_sequence_number: 0,
            subscription_events: VecDeque::new(),
        }
    }

    pub fn run(connection: Arc<RwLock<TcpTransport>>, socket: TcpStream) {
        // TOKIO Split the socket into a read / write portion
        // let (mut read, mut write) = stream.split();

        // ENTRY POINT TO ALL OF OPC

        // TOKIO tasks for reading data from the input

        // Future for read has associated callback to store chunk when it's received and
        // when complete message is finished to concatenate it together and process it.

        // Write future writes chunks pending to be sent

        // TOKIO tasks must simulate the loop below, maintaining state and timing out on HELO
        // or inactivity.

        info!("Session started {}", Utc::now());

        // Store the address of the client
        {
            let mut connection = trace_write_lock_unwrap!(connection);
            connection.client_address = Some(socket.peer_addr().unwrap());
            connection.transport_state = TransportState::WaitingHello;
        }

        // Start the subscription timer
        {
            let mut connection = trace_write_lock_unwrap!(connection);
            connection.start_subscription_timer()
        }

        let session_start_time = Utc::now();
        info!("Session started {}", session_start_time);

        // Waiting for hello
        // Hello timeout
        let hello_timeout = {
            let hello_timeout = {
                let connection = trace_read_lock_unwrap!(connection);
                let server_state = trace_read_lock_unwrap!(connection.server_state);
                let server_config = trace_read_lock_unwrap!(server_state.config);
                server_config.tcp_config.hello_timeout as i64
            };
            time::Duration::seconds(hello_timeout)
        };

        // Format of OPC UA TCP is defined in OPC UA Part 6 Chapter 7
        // Basic startup is a HELLO,  OpenSecureChannel, begin

        let (reader, writer) = socket.split();
        let connection_state = ConnectionState {
            connection: connection.clone(),
            read_buffer: MessageBuffer::new(RECEIVE_BUFFER_SIZE),
            bytes_read: 0,
            send_buffer: Cursor::new(vec![0u8; SEND_BUFFER_SIZE]),
            reader,
            writer,
            in_buf: vec![0u8; RECEIVE_BUFFER_SIZE],
            hello_timeout,
            session_start_time,
        };

        Self::execution_loop(connection_state);
    }

    fn is_hello_timeout(connection_state: &ConnectionState) -> bool {
        // Check if the session has waited in the hello state for more than the hello timeout period
        let transport_state = {
            let connection = trace_read_lock_unwrap!(connection_state.connection);
            let server_state = trace_read_lock_unwrap!(connection.server_state);
            connection.transport_state.clone()
        };
        let timeout = if transport_state == TransportState::WaitingHello {
            let now = Utc::now();
            // Check if the time now exceeds the hello time
            if now.signed_duration_since(connection_state.session_start_time.clone()).num_milliseconds() > connection_state.hello_timeout.num_milliseconds() {
                let session_status_code = BadTimeout;
                {
                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                    connection.set_session_status(session_status_code);
                }
                true
            } else {
                false
            }
        } else {
            false
        };
        timeout
    }

    /// This is an asynchronous execution loop run through Tokio's runtime. Each task in the chain
    /// is executed in order asynchronously from a loop that aborts when
    fn execution_loop(connection_state: ConnectionState) {
        // It is asynchronous so the basic flow is this

        // 1. Read bytes
        // 2. Store bytes in a buffer
        // 3. Process any chunks (messages) that can be extracted from buffer
        // 4. Send outgoing messages
        // 5. Terminate if necessary
        // 6. Go to 1
        let looping_task = loop_fn(connection_state, |connection_state| {
            // Check if the session has waited in the hello state for more than the hello timeout period
            /* if Self::is_hello_timeout(&connection_state) {
                let session_status_code = BadTimeout;
                {
                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                    connection.set_session_status(session_status_code);
                }
                Loop::Break(session_status_code)
            } else { */
            // Stuff is taken out of connection state because it is partially consumed by io::read
            let connection = connection_state.connection;
            let read_buffer = connection_state.read_buffer;
            let send_buffer = connection_state.send_buffer;
            let in_buf = connection_state.in_buf;
            let reader = connection_state.reader;
            let writer = connection_state.writer;
            let hello_timeout = connection_state.hello_timeout;
            let session_start_time = connection_state.session_start_time;

            // TODO test for a received publish
            // mpsc::receive a publish response

            // Read and process bytes from the stream
            io::read(reader, in_buf).map_err(|err| {
                error!("Transport IO error {:?}", err);
                BadCommunicationError
            }).map(move |(reader, in_buf, bytes_read)| {
                trace!("Read {} bytes", bytes_read);
                // Build a new connection state
                ConnectionState {
                    connection,
                    read_buffer,
                    bytes_read,
                    send_buffer,
                    reader,
                    writer,
                    in_buf,
                    hello_timeout,
                    session_start_time,
                }
            }).and_then(|mut connection_state| {
                // Check for abort
                let transport_state = {
                    let connection = trace_read_lock_unwrap!(connection_state.connection);
                    let server_state = trace_read_lock_unwrap!(connection.server_state);
                    if server_state.abort {
                        return Err(BadCommunicationError);
                    }
                    connection.transport_state.clone()
                };

                let mut session_status_code = Good;
                let result = connection_state.read_buffer.store_bytes(&connection_state.in_buf[..connection_state.bytes_read]);
                if result.is_err() {
                    session_status_code = result.unwrap_err();
                } else {
                    let messages = result.unwrap();
                    for message in messages {
                        match transport_state {
                            TransportState::WaitingHello => {
                                debug!("Processing HELLO");
                                if let Message::Hello(hello) = message {
                                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                                    let result = connection.process_hello(hello, &mut connection_state.send_buffer);
                                    if result.is_err() {
                                        session_status_code = result.unwrap_err();
                                    }
                                } else {
                                    session_status_code = BadCommunicationError;
                                }
                            }
                            TransportState::ProcessMessages => {
                                debug!("Processing message");
                                if let Message::MessageChunk(chunk) = message {
                                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                                    let result = connection.process_chunk(chunk, &mut connection_state.send_buffer);
                                    if result.is_err() {
                                        session_status_code = result.unwrap_err();
                                    }
                                } else {
                                    session_status_code = BadCommunicationError;
                                }
                            }
                            _ => {
                                error!("Unknown sesion state, aborting");
                                session_status_code = BadUnexpectedError;
                            }
                        };
                    }
                }

                // Update the session status
                {
                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                    connection.set_session_status(session_status_code);
                }

                Ok(connection_state)
            }).and_then(|mut connection_state| {
                // Process subscription timer events
                let subscription_event = {
                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                    connection.receive_subscription_event()
                };
                if let Some(subscription_event) = subscription_event {
                    match subscription_event {
                        SubscriptionEvent::PublishResponses(publish_responses) => {
                            trace!("Got {} PublishResponse messages to send", publish_responses.len());
                            for publish_response in publish_responses {
                                trace!("<-- Sending a Publish Response{}, {:?}", publish_response.request_id, &publish_response.response);
                                let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                                let _ = connection.send_response(publish_response.request_id, &publish_response.response, &mut connection_state.send_buffer);
                            }
                        }
                    }
                }

                // Write any output
                let _ = Self::write_output(&mut connection_state);

                Ok(connection_state)
            }).and_then(|mut connection_state| {
                // Some handlers might wish to send their message and terminate, in which case this is
                // done here.
                let session_status = {
                    let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                    // Terminate may have been set somewhere
                    let terminate_session = {
                        let session = trace_read_lock_unwrap!(connection.session);
                        session.terminate_session
                    };
                    if terminate_session {
                        connection.set_session_status(BadConnectionClosed);
                    }
                    // Other session status
                    connection.session_status()
                };

                // Abort the session?
                if session_status.is_good() {
                    Ok(Loop::Continue(connection_state))
                } else {
                    /*
                TODO
                drop(subscription_timer_guard);
                drop(subscription_timer);
                */

                    // As a final act, the session sends a status code to the client if one should be sent
                    match session_status {
                        Good | BadConnectionClosed => {
                            info!("Session terminating normally, session_status_code = {:?}", session_status);
                        }
                        _ => {
                            warn!("Sending session terminating error {:?}", session_status);
                            {
                                let out_buf_stream = &mut connection_state.send_buffer;
                                out_buf_stream.set_position(0);
                                let error = ErrorMessage::from_status_code(session_status);
                                let _ = error.encode(out_buf_stream);
                            }
                            let _ = Self::write_output(&mut connection_state);
                        }
                    }

                    // Session state
                    {
                        let mut connection = trace_write_lock_unwrap!(connection_state.connection);
                        connection.transport_state = TransportState::Finished;
                    }

                    let session_duration = Utc::now().signed_duration_since(connection_state.session_start_time);
                    info!("Session is finished {:?}", session_duration);

                    {
                        let connection = trace_read_lock_unwrap!(connection_state.connection);
                        let mut session = trace_write_lock_unwrap!(connection.session);
                        session.set_terminated();
                    }

                    // Abort
                    Ok(Loop::Break(session_status))
                }
            })
//            }
        }).map_err(|err| ()).map(|r| ());
        // Spawn the looping task
        tokio::spawn(looping_task);
    }

    /// Test if the connection is terminated
    pub fn terminated(&self) -> bool {
        if let Ok(ref session) = self.session.try_read() {
            session.terminated()
        } else {
            false
        }
    }

    /// Start the subscription timer to service subscriptions
    fn start_subscription_timer(&mut self) {
        let (subscription_timer_tx, subscription_timer_rx) = mpsc::channel();

        let session = self.session.clone();
        let address_space = self.address_space.clone();

        // Creates a repeating timer that checks subscriptions. The guard is returned to the caller
        // so it can control the scope of events.
        let subscription_timer = timer::Timer::new();
        let subscription_timer_guard = subscription_timer.schedule_repeating(time::Duration::milliseconds(constants::SUBSCRIPTION_TIMER_RATE_MS), move || {
            let mut session = trace_write_lock_unwrap!(session);
            let now = Utc::now();

            // Request queue might contain stale publish requests
            session.expire_stale_publish_requests(&now);

            // Process subscriptions
            {
                let address_space = trace_read_lock_unwrap!(address_space);
                let _ = session.tick_subscriptions(&now, &address_space, TickReason::TickTimerFired);
            }

            // Check if there are publish responses to send for transmission
            if !session.subscriptions.publish_response_queue.is_empty() {
                trace!("Sending publish responses to session thread");
                let mut publish_responses = VecDeque::with_capacity(session.subscriptions.publish_response_queue.len());
                publish_responses.append(&mut session.subscriptions.publish_response_queue);
                drop(session);
                let sent = subscription_timer_tx.send(SubscriptionEvent::PublishResponses(publish_responses));
                if sent.is_err() {
                    error!("Can't send publish responses, err = {}", sent.unwrap_err());
                }
            }
        });
    }

    fn stop_subscription_timer(&mut self) {
        // Just drop the timers
    }

    fn receive_subscription_event(&mut self) -> Option<SubscriptionEvent> {
//        if let Some((_, _, ref mut subscription_timer_rx)) = self.subscription_timer {
//            if let Ok(event) = subscription_timer_rx.try_recv() {
//                Some(event)
//            } else {
//                None
//            }
//        } else {
        None
//        }
    }

    fn write_output(connection_state: &mut ConnectionState) -> std::io::Result<usize> {
        if connection_state.send_buffer.position() == 0 {
            Ok(0)
        } else {
            let result = {
                let out_buf_stream = &connection_state.send_buffer;
                let bytes_to_write = out_buf_stream.position() as usize;
                let buffer_slice = &out_buf_stream.get_ref()[0..bytes_to_write];

                trace!("Writing {} bytes to client", buffer_slice.len());
                // log_buffer("Writing bytes to client:", buffer_slice);

                let result = connection_state.writer.write(&buffer_slice);
                match result {
                    Err(err) => {
                        error!("Error writing bytes - {:?}", err);
                        Err(err)
                    }
                    Ok(bytes_written) => {
                        if bytes_to_write != bytes_written {
                            error!("Error writing bytes - bytes_to_write = {}, bytes_written = {}", bytes_to_write, bytes_written);
                        } else {
                            trace!("Bytes written = {}", bytes_written);
                        }
                        Ok(bytes_written)
                    }
                }
            };

            // AND THEN clear the buffer
            connection_state.send_buffer.set_position(0);

            result
        }
    }

    fn process_hello<W: Write>(&mut self, hello: HelloMessage, out_stream: &mut W) -> std::result::Result<(), StatusCode> {
        let server_protocol_version = 0;

        trace!("Server received HELLO {:?}", hello);
        if !hello.is_endpoint_url_valid() {
            return Err(BadTcpEndpointUrlInvalid);
        }
        if !hello.is_valid_buffer_sizes() {
            error!("HELLO buffer sizes are invalid");
            return Err(BadCommunicationError);
        }

        // Validate protocol version
        if hello.protocol_version > server_protocol_version {
            return Err(BadProtocolVersionUnsupported);
        }

        let client_protocol_version = hello.protocol_version;

        // Send acknowledge
        let mut acknowledge = AcknowledgeMessage {
            message_header: MessageHeader::new(MessageType::Acknowledge),
            protocol_version: server_protocol_version,
            receive_buffer_size: RECEIVE_BUFFER_SIZE as UInt32,
            send_buffer_size: SEND_BUFFER_SIZE as UInt32,
            max_message_size: MAX_MESSAGE_SIZE as UInt32,
            max_chunk_count: MAX_CHUNK_COUNT as UInt32,
        };
        acknowledge.message_header.message_size = acknowledge.byte_len() as UInt32;

        // New state
        self.transport_state = TransportState::ProcessMessages;
        self.client_protocol_version = client_protocol_version;

        debug!("Sending ACK");
        let _ = acknowledge.encode(out_stream);
        Ok(())
    }

    fn turn_received_chunks_into_message(&mut self, chunks: &Vec<MessageChunk>) -> std::result::Result<SupportedMessage, StatusCode> {
        let session = trace_read_lock_unwrap!(self.session);
        // Validate that all chunks have incrementing sequence numbers and valid chunk types
        self.last_received_sequence_number = Chunker::validate_chunks(self.last_received_sequence_number + 1, &session.secure_channel, chunks)?;
        // Now decode
        Chunker::decode(&chunks, &session.secure_channel, None)
    }

    fn process_chunk<W: Write>(&mut self, chunk: MessageChunk, out_stream: &mut W) -> std::result::Result<(), StatusCode> {
        let message_header = chunk.message_header()?;

        if message_header.is_final == MessageIsFinalType::Intermediate {
            panic!("We don't support intermediate chunks yet");
        } else if message_header.is_final == MessageIsFinalType::FinalError {
            info!("Discarding chunk marked in as final error");
            return Ok(());
        }

        // Decrypt / verify chunk if necessary
        let chunk = {
            let mut session = trace_write_lock_unwrap!(self.session);
            session.secure_channel.verify_and_remove_security(&chunk.data)?
        };

        let in_chunks = vec![chunk];
        let chunk_info = {
            let session = trace_read_lock_unwrap!(self.session);
            in_chunks[0].chunk_info(&session.secure_channel)?
        };
        let request_id = chunk_info.sequence_header.request_id;

        let message = self.turn_received_chunks_into_message(&in_chunks)?;
        let response = match message_header.message_type {
            MessageChunkType::OpenSecureChannel => {
                let mut session = trace_write_lock_unwrap!(self.session);
                self.secure_channel_service.open_secure_channel(&mut session.secure_channel, &chunk_info.security_header, self.client_protocol_version, &message)?
            }
            MessageChunkType::CloseSecureChannel => {
                self.secure_channel_service.close_secure_channel(&message)?
            }
            MessageChunkType::Message => {
                let response = self.message_handler.handle_message(request_id, message)?;
                if response.is_none() {
                    // No response for the message at this time
                    return Ok(());
                }
                response.unwrap()
            }
        };
        self.send_response(request_id, &response, out_stream)?;
        Ok(())
    }

    fn send_response<W: Write>(&mut self, request_id: UInt32, response: &SupportedMessage, out_stream: &mut W) -> std::result::Result<(), StatusCode> {
        // Prepare some chunks starting from the sequence number + 1
        match *response {
            SupportedMessage::Invalid(object_id) => {
                panic!("Invalid response with object_id {:?}", object_id);
            }
            _ => {
                // Send the response
                // Get the request id out of the request
                // debug!("Response to send: {:?}", response);
                let sequence_number = self.last_sent_sequence_number + 1;
                // TODO max message size, max chunk size
                let max_chunk_size = 64 * 1024;
                let out_chunks = {
                    let session = trace_read_lock_unwrap!(self.session);
                    Chunker::encode(sequence_number, request_id, 0, max_chunk_size, &session.secure_channel, response)?
                };
                self.last_sent_sequence_number = sequence_number + out_chunks.len() as UInt32 - 1;

                // Send out any chunks that form the response
                // debug!("Got some chunks to send {:?}", out_chunks);
                let mut data = vec![0u8; max_chunk_size + 1024];
                for out_chunk in &out_chunks {
                    // Encrypt and sign the chunk if necessary
                    let session = trace_read_lock_unwrap!(self.session);
                    let size = session.secure_channel.apply_security(out_chunk, &mut data);
                    if size.is_ok() {
                        let _ = out_stream.write(&data[..size.unwrap()]);
                    } else {
                        panic!("Applying security to chunk failed - {:?}", size.unwrap_err());
                    }
                }
            }
        }
        Ok(())
    }
}


