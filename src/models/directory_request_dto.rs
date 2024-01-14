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
pub struct DirectoryRequestDto {
    #[serde(rename = "path")]
    pub path: String,
}

impl DirectoryRequestDto {
    pub fn new(path: String) -> DirectoryRequestDto {
        DirectoryRequestDto {
            path,
        }
    }
}

