// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct EndpointDescription {
    pub endpoint_url: opcua::types::string::UAString,
    pub server: super::application_description::ApplicationDescription,
    pub server_certificate: opcua::types::byte_string::ByteString,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_policy_uri: opcua::types::string::UAString,
    pub user_identity_tokens: Option<Vec<super::user_token_policy::UserTokenPolicy>>,
    pub transport_profile_uri: opcua::types::string::UAString,
    pub security_level: u8,
}
impl opcua::types::MessageInfo for EndpointDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointDescription_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::EndpointDescription
    }
}