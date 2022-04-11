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
    service_types::enums::JsonNetworkMessageContentMask,
    service_types::enums::JsonDataSetMessageContentMask,
};

#[derive(Debug, Clone, PartialEq)]
pub struct JsonDataSetReaderMessageDataType {
    pub network_message_content_mask: JsonNetworkMessageContentMask,
    pub data_set_message_content_mask: JsonDataSetMessageContentMask,
}

impl BinaryEncoder<JsonDataSetReaderMessageDataType> for JsonDataSetReaderMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.network_message_content_mask.byte_len();
        size += self.data_set_message_content_mask.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.network_message_content_mask.encode(stream)?;
        size += self.data_set_message_content_mask.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let network_message_content_mask = JsonNetworkMessageContentMask::decode(stream, decoding_options)?;
        let data_set_message_content_mask = JsonDataSetMessageContentMask::decode(stream, decoding_options)?;
        Ok(JsonDataSetReaderMessageDataType {
            network_message_content_mask,
            data_set_message_content_mask,
        })
    }
}