use crate::{
    BulkString, RespArray, RespDecode, RespError, RespFrame, RespMap, RespNull, RespNullArray,
    RespNullBulkString, RespSet, SimpleError, SimpleString,
};

use bytes::{Bytes, BytesMut};

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

impl RespDecode for SimpleString {
    const PREFIX: &'static [u8] = "+";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = 
    }
}


fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str
) -> Result<(), RespError> {
    
}