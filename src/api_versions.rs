use crate::describe_topic_partitions::describe_topic_partitions_keys;

pub struct ApiKeys {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}

pub fn check_valid_api_version(version: i16) -> [u8; 2] {
    println!("checking api version");
    let error_code: i16 = if (0..=4).contains(&version) { 0 } else { 35 };
    let error_code_bytes: [u8; 2] = i16::to_be_bytes(error_code);
    error_code_bytes
}

pub fn serialize_api_keys(api_keys: &[ApiKeys]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.push((api_keys.len() + 1) as u8);
    for api_key in api_keys {
        out.extend_from_slice(&api_key.api_key.to_be_bytes());
        out.extend_from_slice(&api_key.min_version.to_be_bytes());
        out.extend_from_slice(&api_key.max_version.to_be_bytes());
        out.push(0u8);
    }
    out
}

pub fn api_versions_key() -> ApiKeys {
    ApiKeys {
        api_key: 18,
        min_version: 0,
        max_version: 4,
    }
}
