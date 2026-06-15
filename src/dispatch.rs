use crate::api_versions::{ApiKeys, api_versions_key, check_valid_api_version, serialize_api_keys};
use crate::describe_topic_partitions::describe_topic_partitions_keys;

pub fn build_response(payload: Vec<u8>) -> Vec<u8> {
    println!("building response");
    let correlation_id_bytes: [u8; 4] = payload[4..8].try_into().unwrap();
    // let correlation_id = i32::from_be_bytes(correlation_id_bytes);
    // Convert big-endian bytes to 32-bits integer
    let api_keys_arr = api_versions_response();
    let api_keys_byte = serialize_api_keys(&api_keys_arr);
    let api_version_bytes: [u8; 2] = payload[2..4].try_into().unwrap();
    let api_version = i16::from_be_bytes(api_version_bytes);
    let error_code: [u8; 2] = check_valid_api_version(api_version);

    let throttle_time_ms: i32 = 0; // Placeholder
    let tagged_fields: i8 = 0; // Placeholder

    // Build the [u8; 8] but since this project will grow I will use Vec<u8>
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(&correlation_id_bytes);
    body.extend_from_slice(&error_code);
    body.extend_from_slice(&api_keys_byte);
    body.extend_from_slice(&throttle_time_ms.to_be_bytes());
    body.extend_from_slice(&tagged_fields.to_be_bytes());

    let message_size: i32 = body.len() as i32;

    let mut response: Vec<u8> = Vec::new();
    response.extend_from_slice(&message_size.to_be_bytes());
    response.extend_from_slice(&body);
    response
}

fn api_versions_response() -> Vec<ApiKeys> {
    println!("api versions response");
    let available_apis: Vec<fn() -> ApiKeys> =
        vec![api_versions_key, describe_topic_partitions_keys];
    let api_keys: Vec<ApiKeys> = available_apis.iter().map(|build| build()).collect();
    api_keys
}
