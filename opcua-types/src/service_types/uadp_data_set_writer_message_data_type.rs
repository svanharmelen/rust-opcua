// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
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
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct UadpDataSetWriterMessageDataType {
    pub data_set_message_content_mask: super::enums::UadpDataSetMessageContentMask,
    pub configured_size: u16,
    pub network_message_number: u16,
    pub data_set_offset: u16,
}
impl opcua::types::MessageInfo for UadpDataSetWriterMessageDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UadpDataSetWriterMessageDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UadpDataSetWriterMessageDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UadpDataSetWriterMessageDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for UadpDataSetWriterMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_set_message_content_mask.byte_len();
        size += self.configured_size.byte_len();
        size += self.network_message_number.byte_len();
        size += self.data_set_offset.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_set_message_content_mask.encode(stream)?;
        size += self.configured_size.encode(stream)?;
        size += self.network_message_number.encode(stream)?;
        size += self.data_set_offset.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let data_set_message_content_mask = <super::enums::UadpDataSetMessageContentMask as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let configured_size = <u16 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let network_message_number = <u16 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_offset = <u16 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            data_set_message_content_mask,
            configured_size,
            network_message_number,
            data_set_offset,
        })
    }
}
