use crate::api_versions::ApiKeys;

pub fn fetch_keys() -> ApiKeys {
    ApiKeys {
        api_key: 1,
        min_version: 0,
        max_version: 16,
    }
}
