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
pub struct ReadListUpdateDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "bookIds", skip_serializing_if = "Option::is_none")]
    pub book_ids: Option<Vec<String>>,
    #[serde(rename = "ordered", skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
}

impl ReadListUpdateDto {
    pub fn new() -> ReadListUpdateDto {
        ReadListUpdateDto {
            name: None,
            summary: None,
            book_ids: None,
            ordered: None,
        }
    }
}


