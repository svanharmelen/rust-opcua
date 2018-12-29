// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    service_types::impls::ResponseHeader,
    diagnostic_info::DiagnosticInfo,
    service_types::BrowsePathResult,
};

/// Translates one or more paths in the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslateBrowsePathsToNodeIdsResponse {
    pub response_header: ResponseHeader,
    pub results: Option<Vec<BrowsePathResult>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for TranslateBrowsePathsToNodeIdsResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::TranslateBrowsePathsToNodeIdsResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TranslateBrowsePathsToNodeIdsResponse> for TranslateBrowsePathsToNodeIdsResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_limits)?;
        let results: Option<Vec<BrowsePathResult>> = read_array(stream, decoding_limits)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_limits)?;
        Ok(TranslateBrowsePathsToNodeIdsResponse {
            response_header,
            results,
            diagnostic_infos,
        })
    }
}
