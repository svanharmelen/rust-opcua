// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[derive(Default)]
pub struct EventFilterResult {
    pub select_clause_results: Option<Vec<opcua::types::status_code::StatusCode>>,
    pub select_clause_diagnostic_infos: Option<
        Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
    >,
    pub where_clause_result: super::content_filter_result::ContentFilterResult,
}
impl opcua::types::MessageInfo for EventFilterResult {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilterResult_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for EventFilterResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.select_clause_results.byte_len();
        size += self.select_clause_diagnostic_infos.byte_len();
        size += self.where_clause_result.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.select_clause_results.encode(stream)?;
        size += self.select_clause_diagnostic_infos.encode(stream)?;
        size += self.where_clause_result.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let select_clause_results = <Option<
            Vec<opcua::types::status_code::StatusCode>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let select_clause_diagnostic_infos = <Option<
            Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let where_clause_result = <super::content_filter_result::ContentFilterResult as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            select_clause_results,
            select_clause_diagnostic_infos,
            where_clause_result,
        })
    }
}
