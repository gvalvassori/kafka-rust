use crate::describe_topic_partitions::describe_topic_partitions_keys;
use crate::encoder::Encode;
use crate::fetch::fetch_keys;
use crate::produce::produce_keys;

pub struct ApiKeys {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}

impl ApiKeys {
    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(&self.api_key.to_be_bytes());
        out.extend_from_slice(&self.min_version.to_be_bytes());
        out.extend_from_slice(&self.max_version.to_be_bytes());
        out.push(0u8);
        out
    }
}

pub struct ApiVersionsResponse {
    error_code: i16,
    api_keys: Vec<ApiKeys>,
}

fn check_valid_api_version(version: i16) -> i16 {
    println!("checking api version");
    let error_code: i16 = if (0..=4).contains(&version) { 0 } else { 35 };
    error_code
}

pub fn build_apiversions_response(api_version: i16) -> ApiVersionsResponse {
    let error_code = check_valid_api_version(api_version);
    let api_keys = api_versions_response();
    ApiVersionsResponse {
        error_code,
        api_keys,
    }
}

fn api_versions_response() -> Vec<ApiKeys> {
    println!("api versions response");
    let available_apis: Vec<fn() -> ApiKeys> = vec![
        api_versions_key,
        describe_topic_partitions_keys,
        fetch_keys,
        produce_keys,
    ];
    let api_keys: Vec<ApiKeys> = available_apis.iter().map(|build| build()).collect();
    api_keys
}

fn api_versions_key() -> ApiKeys {
    ApiKeys {
        api_key: 18,
        min_version: 0,
        max_version: 4,
    }
}

impl Encode for ApiVersionsResponse {
    fn encode(&self, correlation_id: i32) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        encoded.extend_from_slice(&correlation_id.to_be_bytes());
        // encoded.push(0u8); Comented out this is for v1 response, not needed now
        encoded.extend_from_slice(&self.error_code.to_be_bytes());
        let api_keys_len = self.api_keys.len();
        encoded.push((api_keys_len + 1) as u8);
        for api_key in &self.api_keys {
            encoded.extend_from_slice(&api_key.to_bytes());
        }
        // TODO: remove hardcoded throttle_time_ms
        let throttle_time_ms: i32 = 0;
        encoded.extend_from_slice(&throttle_time_ms.to_be_bytes());
        encoded.push(0u8);
        encoded
    }
}
