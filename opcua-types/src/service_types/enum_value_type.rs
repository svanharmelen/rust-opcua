// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct EnumValueType {
    pub value: i64,
    pub display_name: opcua::types::localized_text::LocalizedText,
    pub description: opcua::types::localized_text::LocalizedText,
}
impl opcua::types::MessageInfo for EnumValueType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EnumValueType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for EnumValueType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.value.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.value.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let value = <i64 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let display_name = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let description = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            value,
            display_name,
            description,
        })
    }
}
