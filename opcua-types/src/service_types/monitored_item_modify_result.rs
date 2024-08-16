// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct MonitoredItemModifyResult {
    pub status_code: opcua::types::status_code::StatusCode,
    pub revised_sampling_interval: f64,
    pub revised_queue_size: u32,
    pub filter_result: opcua::types::extension_object::ExtensionObject,
}
impl opcua::types::MessageInfo for MonitoredItemModifyResult {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MonitoredItemModifyResult_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for MonitoredItemModifyResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.status_code.byte_len();
        size += self.revised_sampling_interval.byte_len();
        size += self.revised_queue_size.byte_len();
        size += self.filter_result.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.status_code.encode(stream)?;
        size += self.revised_sampling_interval.encode(stream)?;
        size += self.revised_queue_size.encode(stream)?;
        size += self.filter_result.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let status_code = <opcua::types::status_code::StatusCode as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let revised_sampling_interval = <f64 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let revised_queue_size = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let filter_result = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            status_code,
            revised_sampling_interval,
            revised_queue_size,
            filter_result,
        })
    }
}
