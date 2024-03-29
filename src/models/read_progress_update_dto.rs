/*
 * Komga API
 *
 * Komga offers 2 APIs: REST and OPDS.  Both APIs are secured using HTTP Basic Authentication.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReadProgressUpdateDto : page can be omitted if completed is set to true. completed can be omitted, and will be set accordingly depending on the page passed and the total number of pages in the book.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadProgressUpdateDto {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

impl ReadProgressUpdateDto {
    /// page can be omitted if completed is set to true. completed can be omitted, and will be set accordingly depending on the page passed and the total number of pages in the book.
    pub fn new() -> ReadProgressUpdateDto {
        ReadProgressUpdateDto {
            page: None,
            completed: None,
        }
    }
}


