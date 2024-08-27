// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the `BinaryEncoder` trait and helpers for reading and writing of scalar values and
//! other primitives.

use std::{
    fmt::{Debug, Display},
    io::{Cursor, Read, Result, Write},
    sync::atomic::{AtomicU64, Ordering},
};

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use chrono::Duration;
use log::{error, trace, warn};

use crate::{constants, status_code::StatusCode};

pub type EncodingResult<T> = std::result::Result<T, EncodingError>;

#[derive(Debug, Clone, Copy)]
pub struct EncodingError {
    status: StatusCode,
    request_id: Option<u32>,
    request_handle: Option<u32>,
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status())
    }
}

impl From<StatusCode> for EncodingError {
    fn from(value: StatusCode) -> Self {
        Self {
            status: value,
            request_handle: None,
            request_id: None,
        }
    }
}

impl EncodingError {
    pub fn new(status: StatusCode, request_id: Option<u32>, request_handle: Option<u32>) -> Self {
        Self {
            status,
            request_handle,
            request_id,
        }
    }

    pub fn with_context(mut self, id: Option<u32>, handle: Option<u32>) -> Self {
        self.request_id = id;
        self.request_handle = handle;
        self
    }

    pub fn with_request_id(mut self, id: u32) -> Self {
        self.request_id = Some(id);
        self
    }

    pub fn with_request_handle(mut self, handle: u32) -> Self {
        self.request_handle = Some(handle);
        self
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn full_context(&self) -> Option<(u32, u32)> {
        if let (Some(id), Some(handle)) = (self.request_id, self.request_handle) {
            Some((id, handle))
        } else {
            None
        }
    }
}

impl From<EncodingError> for StatusCode {
    fn from(value: EncodingError) -> Self {
        value.status()
    }
}

impl From<EncodingError> for std::io::Error {
    fn from(value: EncodingError) -> Self {
        value.status().into()
    }
}

/// Depth lock holds a reference on the depth gauge. The drop ensures impl that the reference is
/// decremented even if there is a panic unwind.
#[derive(Debug)]
pub struct DepthLock<'a> {
    depth_gauge: &'a DepthGauge,
}

impl<'a> Drop for DepthLock<'a> {
    fn drop(&mut self) {
        // This will overflow back if the gauge is somehow at 0. That really should not be possible, if it is only ever
        // incremented from `obtain`
        self.depth_gauge
            .current_depth
            .fetch_sub(1, Ordering::Release);
    }
}

impl<'a> DepthLock<'a> {
    fn new(depth_gauge: &'a DepthGauge) -> (Self, u64) {
        let current = depth_gauge.current_depth.fetch_add(1, Ordering::Acquire);

        (Self { depth_gauge }, current)
    }

    /// The depth lock tests if the depth can increment and then obtains a lock on it.
    /// The lock will decrement the depth when it drops to ensure proper behaviour during unwinding.
    pub fn obtain(depth_gauge: &'a DepthGauge) -> core::result::Result<DepthLock<'a>, StatusCode> {
        let max_depth = depth_gauge.max_depth;
        let (gauge, val) = Self::new(depth_gauge);

        if val >= max_depth {
            warn!("Decoding in stream aborted due maximum recursion depth being reached");
            Err(StatusCode::BadDecodingError)
        } else {
            Ok(gauge)
        }
    }
}

/// Depth gauge is used on potentially recursive structures like Variant & ExtensionObject during
/// decoding to limit the depth the decoder will go before giving up.
#[derive(Debug)]
pub struct DepthGauge {
    /// Maximum decoding depth for recursive elements. Triggers when current depth equals max depth.
    pub(self) max_depth: u64,
    /// Current decoding depth for recursive elements.
    pub(self) current_depth: AtomicU64,
}

// TODO: In general keeping DepthGauge as part of DecodingOptions is suboptimal,
// since this pattern is unintuitive. It should be separated out.
impl Clone for DepthGauge {
    fn clone(&self) -> Self {
        Self {
            max_depth: self.max_depth.clone(),
            current_depth: AtomicU64::new(0),
        }
    }
}

impl Default for DepthGauge {
    fn default() -> Self {
        Self::new(constants::MAX_DECODING_DEPTH)
    }
}

impl DepthGauge {
    pub fn new(max_depth: u64) -> Self {
        Self {
            max_depth,
            current_depth: AtomicU64::new(0),
        }
    }

    pub fn minimal() -> Self {
        Self {
            max_depth: 1,
            ..Default::default()
        }
    }
    pub fn max_depth(&self) -> u64 {
        self.max_depth
    }
}

#[derive(Clone, Debug)]
pub struct DecodingOptions {
    /// Time offset between the client and the server, only used by the client when it's configured
    /// to ignore time skew.
    pub client_offset: Duration,
    /// Maximum size of a message in bytes. 0 means no limit.
    pub max_message_size: usize,
    /// Maximum number of chunks. 0 means no limit.
    pub max_chunk_count: usize,
    /// Maximum length in bytes (not chars!) of a string. 0 actually means 0, i.e. no string permitted
    pub max_string_length: usize,
    /// Maximum length in bytes of a byte string. 0 actually means 0, i.e. no byte string permitted
    pub max_byte_string_length: usize,
    /// Maximum number of array elements. 0 actually means 0, i.e. no array permitted
    pub max_array_length: usize,
    /// Decoding depth gauge is used to check for recursion
    pub decoding_depth_gauge: DepthGauge,
}

impl Default for DecodingOptions {
    fn default() -> Self {
        DecodingOptions {
            client_offset: Duration::zero(),
            max_message_size: constants::MAX_MESSAGE_SIZE,
            max_chunk_count: constants::MAX_CHUNK_COUNT,
            max_string_length: constants::MAX_STRING_LENGTH,
            max_byte_string_length: constants::MAX_BYTE_STRING_LENGTH,
            max_array_length: constants::MAX_ARRAY_LENGTH,
            decoding_depth_gauge: DepthGauge::default(),
        }
    }
}

impl DecodingOptions {
    /// This can be useful for decoding extension objects where the payload is not expected to contain
    /// a large value.
    pub fn minimal() -> Self {
        DecodingOptions {
            max_string_length: 8192,
            max_byte_string_length: 8192,
            max_array_length: 8192,
            decoding_depth_gauge: DepthGauge::minimal(),
            ..Default::default()
        }
    }

    /// For test only. Having a separate function makes it easier to control calls to DecodingOptions::default().
    pub fn test() -> Self {
        Self::default()
    }

    pub fn depth_lock<'a>(&'a self) -> core::result::Result<DepthLock<'a>, StatusCode> {
        DepthLock::obtain(&self.decoding_depth_gauge)
    }
}

/// OPC UA Binary Encoding interface. Anything that encodes to binary must implement this. It provides
/// functions to calculate the size in bytes of the struct (for allocating memory), encoding to a stream
/// and decoding from a stream.
pub trait BinaryEncoder: Sized {
    /// Returns the exact byte length of the structure as it would be if `encode` were called.
    /// This may be called prior to writing to ensure the correct amount of space is available.
    fn byte_len(&self) -> usize;
    /// Encodes the instance to the write stream.
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize>;
    /// Decodes an instance from the read stream. The decoding options contains restrictions set by
    /// the server / client on the length of strings, arrays etc. If these limits are exceeded the
    /// implementation should return with a `BadDecodingError` as soon as possible.
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self>;

    // Convenience method for encoding a message straight into an array of bytes. It is preferable to reuse buffers than
    // to call this so it should be reserved for tests and trivial code.
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut buffer = Cursor::new(Vec::with_capacity(self.byte_len()));
        let _ = self.encode(&mut buffer);
        buffer.into_inner()
    }
}

/// Converts an IO encoding error (and logs when in error) into an EncodingResult
pub fn process_encode_io_result(result: Result<usize>) -> EncodingResult<usize> {
    result.map_err(|err| {
        trace!("Encoding error - {:?}", err);
        StatusCode::BadEncodingError.into()
    })
}

/// Converts an IO encoding error (and logs when in error) into an EncodingResult
pub fn process_decode_io_result<T>(result: Result<T>) -> EncodingResult<T>
where
    T: Debug,
{
    result.map_err(|err| {
        trace!("Decoding error - {:?}", err);
        StatusCode::BadDecodingError.into()
    })
}

impl<T> BinaryEncoder for Option<Vec<T>>
where
    T: BinaryEncoder,
{
    fn byte_len(&self) -> usize {
        let mut size = 4;
        if let Some(ref values) = self {
            size += values.iter().map(|v| v.byte_len()).sum::<usize>();
        }
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        if let Some(ref values) = self {
            size += write_i32(stream, values.len() as i32)?;
            for value in values.iter() {
                size += value.encode(stream)?;
            }
        } else {
            size += write_i32(stream, -1)?;
        }
        Ok(size)
    }

    fn decode<S: Read>(
        stream: &mut S,
        decoding_options: &DecodingOptions,
    ) -> EncodingResult<Option<Vec<T>>> {
        let len = read_i32(stream)?;
        if len == -1 {
            Ok(None)
        } else if len < -1 {
            error!("Array length is negative value and invalid");
            Err(StatusCode::BadDecodingError.into())
        } else if len as usize > decoding_options.max_array_length {
            error!(
                "Array length {} exceeds decoding limit {}",
                len, decoding_options.max_array_length
            );
            Err(StatusCode::BadDecodingError.into())
        } else {
            let mut values: Vec<T> = Vec::with_capacity(len as usize);
            for _ in 0..len {
                values.push(T::decode(stream, decoding_options)?);
            }
            Ok(Some(values))
        }
    }
}

/// Calculates the length in bytes of an array of encoded type
pub fn byte_len_array<T: BinaryEncoder>(values: &Option<Vec<T>>) -> usize {
    let mut size = 4;
    if let Some(ref values) = values {
        size += values.iter().map(|v| v.byte_len()).sum::<usize>();
    }
    size
}

/// Write an array of the encoded type to stream, preserving distinction between null array and empty array
pub fn write_array<S: Write, T: BinaryEncoder>(
    stream: &mut S,
    values: &Option<Vec<T>>,
) -> EncodingResult<usize> {
    let mut size = 0;
    if let Some(ref values) = values {
        size += write_i32(stream, values.len() as i32)?;
        for value in values.iter() {
            size += value.encode(stream)?;
        }
    } else {
        size += write_i32(stream, -1)?;
    }
    Ok(size)
}

/// Reads an array of the encoded type from a stream, preserving distinction between null array and empty array
pub fn read_array<S: Read, T: BinaryEncoder>(
    stream: &mut S,
    decoding_options: &DecodingOptions,
) -> EncodingResult<Option<Vec<T>>> {
    let len = read_i32(stream)?;
    if len == -1 {
        Ok(None)
    } else if len < -1 {
        error!("Array length is negative value and invalid");
        Err(StatusCode::BadDecodingError.into())
    } else if len as usize > decoding_options.max_array_length {
        error!(
            "Array length {} exceeds decoding limit {}",
            len, decoding_options.max_array_length
        );
        Err(StatusCode::BadDecodingError.into())
    } else {
        let mut values: Vec<T> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            values.push(T::decode(stream, decoding_options)?);
        }
        Ok(Some(values))
    }
}

/// Writes a series of identical bytes to the stream
pub fn write_bytes(stream: &mut dyn Write, value: u8, count: usize) -> EncodingResult<usize> {
    for _ in 0..count {
        let _ = stream
            .write_u8(value)
            .map_err(|_| StatusCode::BadEncodingError)?;
    }
    Ok(count)
}

/// Writes an unsigned byte to the stream
pub fn write_u8<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<u8>,
{
    let buf: [u8; 1] = [value.into()];
    process_encode_io_result(stream.write(&buf))
}

/// Writes a signed 16-bit value to the stream
pub fn write_i16<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<i16>,
{
    let mut buf = [0u8; 2];
    LittleEndian::write_i16(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes an unsigned 16-bit value to the stream
pub fn write_u16<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<u16>,
{
    let mut buf = [0u8; 2];
    LittleEndian::write_u16(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes a signed 32-bit value to the stream
pub fn write_i32<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<i32>,
{
    let mut buf = [0u8; 4];
    LittleEndian::write_i32(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes an unsigned 32-bit value to the stream
pub fn write_u32<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<u32>,
{
    let mut buf = [0u8; 4];
    LittleEndian::write_u32(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes a signed 64-bit value to the stream
pub fn write_i64<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<i64>,
{
    let mut buf = [0u8; 8];
    LittleEndian::write_i64(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes an unsigned 64-bit value to the stream
pub fn write_u64<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<u64>,
{
    let mut buf = [0u8; 8];
    LittleEndian::write_u64(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes a 32-bit precision value to the stream
pub fn write_f32<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<f32>,
{
    let mut buf = [0u8; 4];
    LittleEndian::write_f32(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Writes a 64-bit precision value to the stream
pub fn write_f64<T>(stream: &mut dyn Write, value: T) -> EncodingResult<usize>
where
    T: Into<f64>,
{
    let mut buf = [0u8; 8];
    LittleEndian::write_f64(&mut buf, value.into());
    process_encode_io_result(stream.write(&buf))
}

/// Reads an array of bytes from the stream
pub fn read_bytes(stream: &mut dyn Read, buf: &mut [u8]) -> EncodingResult<usize> {
    let result = stream.read_exact(buf);
    process_decode_io_result(result)?;
    Ok(buf.len())
}

/// Read an unsigned byte from the stream
pub fn read_u8(stream: &mut dyn Read) -> EncodingResult<u8> {
    let mut buf = [0u8];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(buf[0])
}

/// Read an signed 16-bit value from the stream
pub fn read_i16(stream: &mut dyn Read) -> EncodingResult<i16> {
    let mut buf = [0u8; 2];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_i16(&buf))
}

/// Read an unsigned 16-bit value from the stream
pub fn read_u16(stream: &mut dyn Read) -> EncodingResult<u16> {
    let mut buf = [0u8; 2];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_u16(&buf))
}

/// Read a signed 32-bit value from the stream
pub fn read_i32(stream: &mut dyn Read) -> EncodingResult<i32> {
    let mut buf = [0u8; 4];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_i32(&buf))
}

/// Read an unsigned 32-bit value from the stream
pub fn read_u32(stream: &mut dyn Read) -> EncodingResult<u32> {
    let mut buf = [0u8; 4];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_u32(&buf))
}

/// Read a signed 64-bit value from the stream
pub fn read_i64(stream: &mut dyn Read) -> EncodingResult<i64> {
    let mut buf = [0u8; 8];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_i64(&buf))
}

/// Read an unsigned 64-bit value from the stream
pub fn read_u64(stream: &mut dyn Read) -> EncodingResult<u64> {
    let mut buf = [0u8; 8];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_u64(&buf))
}

/// Read a 32-bit precision value from the stream
pub fn read_f32(stream: &mut dyn Read) -> EncodingResult<f32> {
    let mut buf = [0u8; 4];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_f32(&buf))
}

/// Read a 64-bit precision from the stream
pub fn read_f64(stream: &mut dyn Read) -> EncodingResult<f64> {
    let mut buf = [0u8; 8];
    let result = stream.read_exact(&mut buf);
    process_decode_io_result(result)?;
    Ok(LittleEndian::read_f64(&buf))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{constants, DepthGauge, DepthLock};
    use crate::StatusCode;

    #[test]
    fn depth_gauge() {
        let dg = Arc::new(DepthGauge::default());

        let max_depth = dg.max_depth();
        assert_eq!(max_depth, constants::MAX_DECODING_DEPTH);

        // Iterate the depth
        {
            let mut v = Vec::new();
            for _ in 0..max_depth {
                v.push(DepthLock::obtain(&dg).unwrap());
            }

            // Depth should now be MAX_DECODING_DEPTH
            {
                assert_eq!(
                    dg.current_depth.load(std::sync::atomic::Ordering::Relaxed),
                    max_depth
                );
            }

            // Next obtain should fail
            assert_eq!(
                DepthLock::obtain(&dg).unwrap_err(),
                StatusCode::BadDecodingError
            );

            // DepthLocks drop here
        }

        // Depth should be zero
        {
            assert_eq!(
                dg.current_depth.load(std::sync::atomic::Ordering::Relaxed),
                0
            );
        }
    }
}