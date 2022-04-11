// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::types::{
    encoding::*,
    basic_types::*,
    localized_text::LocalizedText,
    variant::Variant,
    node_id::NodeId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAttributes {
    pub specified_attributes: u32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub value: Variant,
    pub data_type: NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub access_level: u8,
    pub user_access_level: u8,
    pub minimum_sampling_interval: f64,
    pub historizing: bool,
}

impl BinaryEncoder<VariableAttributes> for VariableAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.value.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += byte_len_array(&self.array_dimensions);
        size += self.access_level.byte_len();
        size += self.user_access_level.byte_len();
        size += self.minimum_sampling_interval.byte_len();
        size += self.historizing.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.value.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += write_array(stream, &self.array_dimensions)?;
        size += self.access_level.encode(stream)?;
        size += self.user_access_level.encode(stream)?;
        size += self.minimum_sampling_interval.encode(stream)?;
        size += self.historizing.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let specified_attributes = u32::decode(stream, decoding_options)?;
        let display_name = LocalizedText::decode(stream, decoding_options)?;
        let description = LocalizedText::decode(stream, decoding_options)?;
        let write_mask = u32::decode(stream, decoding_options)?;
        let user_write_mask = u32::decode(stream, decoding_options)?;
        let value = Variant::decode(stream, decoding_options)?;
        let data_type = NodeId::decode(stream, decoding_options)?;
        let value_rank = i32::decode(stream, decoding_options)?;
        let array_dimensions: Option<Vec<u32>> = read_array(stream, decoding_options)?;
        let access_level = u8::decode(stream, decoding_options)?;
        let user_access_level = u8::decode(stream, decoding_options)?;
        let minimum_sampling_interval = f64::decode(stream, decoding_options)?;
        let historizing = bool::decode(stream, decoding_options)?;
        Ok(VariableAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            value,
            data_type,
            value_rank,
            array_dimensions,
            access_level,
            user_access_level,
            minimum_sampling_interval,
            historizing,
        })
    }
}