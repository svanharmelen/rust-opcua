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
pub struct ThreeDVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl opcua::types::MessageInfo for ThreeDVector {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDVector_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ThreeDVector {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.x.byte_len();
        size += self.y.byte_len();
        size += self.z.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.x.encode(stream)?;
        size += self.y.encode(stream)?;
        size += self.z.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let x = <f64 as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let y = <f64 as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let z = <f64 as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { x, y, z })
    }
}
