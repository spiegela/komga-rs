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
pub struct ReadListRequestBookMatchesDto {
    #[serde(rename = "request")]
    pub request: Box<crate::models::ReadListRequestBookDto>,
    #[serde(rename = "matches")]
    pub matches: Vec<crate::models::ReadListRequestBookMatchDto>,
}

impl ReadListRequestBookMatchesDto {
    pub fn new(request: crate::models::ReadListRequestBookDto, matches: Vec<crate::models::ReadListRequestBookMatchDto>) -> ReadListRequestBookMatchesDto {
        ReadListRequestBookMatchesDto {
            request: Box::new(request),
            matches,
        }
    }
}


