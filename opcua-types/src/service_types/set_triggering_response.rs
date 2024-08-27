// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SetTriggeringResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub add_results: Option<Vec<opcua::types::status_code::StatusCode>>,
    pub add_diagnostic_infos: Option<Vec<opcua::types::diagnostic_info::DiagnosticInfo>>,
    pub remove_results: Option<Vec<opcua::types::status_code::StatusCode>>,
    pub remove_diagnostic_infos: Option<
        Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
    >,
}
impl opcua::types::MessageInfo for SetTriggeringResponse {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SetTriggeringResponse_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for SetTriggeringResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.add_results.byte_len();
        size += self.add_diagnostic_infos.byte_len();
        size += self.remove_results.byte_len();
        size += self.remove_diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.add_results.encode(stream)?;
        size += self.add_diagnostic_infos.encode(stream)?;
        size += self.remove_results.encode(stream)?;
        size += self.remove_diagnostic_infos.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let response_header = <opcua::types::response_header::ResponseHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = response_header.request_handle;
        let add_results = <Option<
            Vec<opcua::types::status_code::StatusCode>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let add_diagnostic_infos = <Option<
            Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let remove_results = <Option<
            Vec<opcua::types::status_code::StatusCode>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let remove_diagnostic_infos = <Option<
            Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            response_header,
            add_results,
            add_diagnostic_infos,
            remove_results,
            remove_diagnostic_infos,
        })
    }
}