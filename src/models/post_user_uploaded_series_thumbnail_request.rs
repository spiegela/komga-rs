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
pub struct PostUserUploadedSeriesThumbnailRequest {
    #[serde(rename = "file")]
    pub file: std::path::PathBuf,
}

impl PostUserUploadedSeriesThumbnailRequest {
    pub fn new(file: std::path::PathBuf) -> PostUserUploadedSeriesThumbnailRequest {
        PostUserUploadedSeriesThumbnailRequest {
            file,
        }
    }
}


