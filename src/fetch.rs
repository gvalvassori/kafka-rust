use crate::api_versions::ApiKeys;
use crate::byte_cursor::Buf;
use crate::encoder::Encode;
use std::io;

pub struct FetchResponse {
    throttle_time_ms: i32,
    error_code: i16,
    session_id: i32,
    responses: Vec<i32>,
}

impl Encode for FetchResponse {
    fn encode(&self, correlation_id: i32) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        encoded.extend_from_slice(&correlation_id.to_be_bytes());
        encoded.push(0u8);
        encoded.extend_from_slice(&self.throttle_time_ms.to_be_bytes());
        encoded.extend_from_slice(&self.error_code.to_be_bytes());
        encoded.extend_from_slice(&self.session_id.to_be_bytes());
        let responses_len = self.responses.len();
        encoded.push((responses_len + 1) as u8); // This is the len of the compact
        encoded.push(0u8);
        encoded.push(0u8); // TODO: This is the TAG_BUFFER, I may need to remove it later
        encoded
    }
}

pub fn build_fetch_response(buf: &mut Buf) -> Result<FetchResponse, io::Error> {
    let throttle_time_ms: i32 = 0; // TODO: placeholder
    let error_code: i16 = 0; // TODO: this is a placeholder for now, replace
    buf.skip(13); // TODO: remove. For now we skip everything till session_id in the request
    let session_id = buf.read_i32();

    Ok(FetchResponse {
        throttle_time_ms,
        error_code,
        session_id,
        responses: vec![],
    })
}

pub fn fetch_keys() -> ApiKeys {
    ApiKeys {
        api_key: 1,
        min_version: 0,
        max_version: 16,
    }
}
