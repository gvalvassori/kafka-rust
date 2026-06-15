use crate::api_versions::ApiKeys;

pub fn describe_topic_partitions_keys() -> ApiKeys {
    ApiKeys {
        api_key: 75,
        min_version: 0,
        max_version: 0,
    }
}
