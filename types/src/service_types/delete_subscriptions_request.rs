// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    service_types::impls::RequestHeader,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteSubscriptionsRequest {
    pub request_header: RequestHeader,
    pub subscription_ids: Option<Vec<u32>>,
}

impl MessageInfo for DeleteSubscriptionsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteSubscriptionsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteSubscriptionsRequest> for DeleteSubscriptionsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.subscription_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.subscription_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let subscription_ids: Option<Vec<u32>> = read_array(stream, decoding_limits)?;
        Ok(DeleteSubscriptionsRequest {
            request_header,
            subscription_ids,
        })
    }
}
