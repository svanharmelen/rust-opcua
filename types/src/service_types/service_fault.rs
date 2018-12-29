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
};

/// The response returned by all services when there is a service level error.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceFault {
    pub response_header: ResponseHeader,
}

impl MessageInfo for ServiceFault {
    fn object_id(&self) -> ObjectId {
        ObjectId::ServiceFault_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ServiceFault> for ServiceFault {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_limits)?;
        Ok(ServiceFault {
            response_header,
        })
    }
}
