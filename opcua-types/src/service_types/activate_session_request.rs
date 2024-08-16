// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct ActivateSessionRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub client_signature: super::signature_data::SignatureData,
    pub client_software_certificates: Option<
        Vec<super::signed_software_certificate::SignedSoftwareCertificate>,
    >,
    pub locale_ids: Option<Vec<opcua::types::string::UAString>>,
    pub user_identity_token: opcua::types::extension_object::ExtensionObject,
    pub user_token_signature: super::signature_data::SignatureData,
}
impl opcua::types::MessageInfo for ActivateSessionRequest {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ActivateSessionRequest_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ActivateSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.client_signature.byte_len();
        size += self.client_software_certificates.byte_len();
        size += self.locale_ids.byte_len();
        size += self.user_identity_token.byte_len();
        size += self.user_token_signature.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.client_signature.encode(stream)?;
        size += self.client_software_certificates.encode(stream)?;
        size += self.locale_ids.encode(stream)?;
        size += self.user_identity_token.encode(stream)?;
        size += self.user_token_signature.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header = <opcua::types::request_header::RequestHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let client_signature = <super::signature_data::SignatureData as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let client_software_certificates = <Option<
            Vec<super::signed_software_certificate::SignedSoftwareCertificate>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let locale_ids = <Option<
            Vec<opcua::types::string::UAString>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let user_identity_token = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let user_token_signature = <super::signature_data::SignatureData as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            client_signature,
            client_software_certificates,
            locale_ids,
            user_identity_token,
            user_token_signature,
        })
    }
}
