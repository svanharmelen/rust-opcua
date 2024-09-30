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
pub struct CallMethodRequest {
    pub object_id: opcua::types::node_id::NodeId,
    pub method_id: opcua::types::node_id::NodeId,
    pub input_arguments: Option<Vec<opcua::types::variant::Variant>>,
}
impl opcua::types::MessageInfo for CallMethodRequest {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CallMethodRequest_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for CallMethodRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.object_id.byte_len();
        size += self.method_id.byte_len();
        size += self.input_arguments.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.object_id.encode(stream)?;
        size += self.method_id.encode(stream)?;
        size += self.input_arguments.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let object_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let method_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let input_arguments = <Option<
            Vec<opcua::types::variant::Variant>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            object_id,
            method_id,
            input_arguments,
        })
    }
}
