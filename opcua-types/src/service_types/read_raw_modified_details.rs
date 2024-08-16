// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct ReadRawModifiedDetails {
    pub is_read_modified: bool,
    pub start_time: opcua::types::date_time::DateTime,
    pub end_time: opcua::types::date_time::DateTime,
    pub num_values_per_node: u32,
    pub return_bounds: bool,
}
impl opcua::types::BinaryEncoder for ReadRawModifiedDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.is_read_modified.byte_len();
        size += self.start_time.byte_len();
        size += self.end_time.byte_len();
        size += self.num_values_per_node.byte_len();
        size += self.return_bounds.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.is_read_modified.encode(stream)?;
        size += self.start_time.encode(stream)?;
        size += self.end_time.encode(stream)?;
        size += self.num_values_per_node.encode(stream)?;
        size += self.return_bounds.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let is_read_modified = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let start_time = <opcua::types::date_time::DateTime as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let end_time = <opcua::types::date_time::DateTime as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let num_values_per_node = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let return_bounds = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            is_read_modified,
            start_time,
            end_time,
            num_values_per_node,
            return_bounds,
        })
    }
}
