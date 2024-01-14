/*
 * Komga API
 *
 * Komga offers 2 APIs: REST and OPDS.  Both APIs are secured using HTTP Basic Authentication.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageHashKnownDto {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "deleteCount")]
    pub delete_count: i32,
    #[serde(rename = "matchCount")]
    pub match_count: i32,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
}

impl PageHashKnownDto {
    pub fn new(hash: String, action: Action, delete_count: i32, match_count: i32, created: String, last_modified: String) -> PageHashKnownDto {
        PageHashKnownDto {
            hash,
            size: None,
            action,
            delete_count,
            match_count,
            created,
            last_modified,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "DELETE_AUTO")]
    DeleteAuto,
    #[serde(rename = "DELETE_MANUAL")]
    DeleteManual,
    #[serde(rename = "IGNORE")]
    Ignore,
}

impl Default for Action {
    fn default() -> Action {
        Self::DeleteAuto
    }
}
