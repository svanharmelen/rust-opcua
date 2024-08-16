// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct RelativePathElement {
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub is_inverse: bool,
    pub include_subtypes: bool,
    pub target_name: opcua::types::qualified_name::QualifiedName,
}
impl opcua::types::MessageInfo for RelativePathElement {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RelativePathElement_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for RelativePathElement {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.reference_type_id.byte_len();
        size += self.is_inverse.byte_len();
        size += self.include_subtypes.byte_len();
        size += self.target_name.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_inverse.encode(stream)?;
        size += self.include_subtypes.encode(stream)?;
        size += self.target_name.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let reference_type_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let is_inverse = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let include_subtypes = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let target_name = <opcua::types::qualified_name::QualifiedName as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            reference_type_id,
            is_inverse,
            include_subtypes,
            target_name,
        })
    }
}
