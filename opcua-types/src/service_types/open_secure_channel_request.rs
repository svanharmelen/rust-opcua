// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
pub struct OpenSecureChannelRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub client_protocol_version: u32,
    pub request_type: super::enums::SecurityTokenRequestType,
    pub security_mode: super::enums::MessageSecurityMode,
    pub client_nonce: opcua::types::byte_string::ByteString,
    pub requested_lifetime: u32,
}
impl opcua::types::MessageInfo for OpenSecureChannelRequest {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for OpenSecureChannelRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.client_protocol_version.byte_len();
        size += self.request_type.byte_len();
        size += self.security_mode.byte_len();
        size += self.client_nonce.byte_len();
        size += self.requested_lifetime.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.client_protocol_version.encode(stream)?;
        size += self.request_type.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.requested_lifetime.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header = <opcua::types::request_header::RequestHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let client_protocol_version = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let request_type = <super::enums::SecurityTokenRequestType as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let security_mode = <super::enums::MessageSecurityMode as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let client_nonce = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let requested_lifetime = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            client_protocol_version,
            request_type,
            security_mode,
            client_nonce,
            requested_lifetime,
        })
    }
}
