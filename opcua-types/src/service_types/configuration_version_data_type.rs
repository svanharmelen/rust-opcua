// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct ConfigurationVersionDataType {
    pub major_version: u32,
    pub minor_version: u32,
}
impl opcua::types::MessageInfo for ConfigurationVersionDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ConfigurationVersionDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ConfigurationVersionDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.major_version.byte_len();
        size += self.minor_version.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.major_version.encode(stream)?;
        size += self.minor_version.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let major_version = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let minor_version = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            major_version,
            minor_version,
        })
    }
}