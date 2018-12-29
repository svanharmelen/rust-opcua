// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    variant::Variant,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralOperand {
    pub value: Variant,
}

impl BinaryEncoder<LiteralOperand> for LiteralOperand {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.value.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.value.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let value = Variant::decode(stream, decoding_limits)?;
        Ok(LiteralOperand {
            value,
        })
    }
}
