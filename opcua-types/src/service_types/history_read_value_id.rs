// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct HistoryReadValueId {
    pub node_id: opcua::types::node_id::NodeId,
    pub index_range: opcua::types::string::UAString,
    pub data_encoding: opcua::types::qualified_name::QualifiedName,
    pub continuation_point: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for HistoryReadValueId {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryReadValueId_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for HistoryReadValueId {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.node_id.byte_len();
        size += self.index_range.byte_len();
        size += self.data_encoding.byte_len();
        size += self.continuation_point.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.node_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.data_encoding.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let node_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let index_range = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_encoding = <opcua::types::qualified_name::QualifiedName as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let continuation_point = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            node_id,
            index_range,
            data_encoding,
            continuation_point,
        })
    }
}
