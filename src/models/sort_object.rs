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
pub struct SortObject {
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "sorted", skip_serializing_if = "Option::is_none")]
    pub sorted: Option<bool>,
    #[serde(rename = "unsorted", skip_serializing_if = "Option::is_none")]
    pub unsorted: Option<bool>,
}

impl SortObject {
    pub fn new() -> SortObject {
        SortObject {
            empty: None,
            sorted: None,
            unsorted: None,
        }
    }
}


