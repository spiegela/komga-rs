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
pub struct ReadListRequestBookDto {
    #[serde(rename = "series")]
    pub series: Vec<String>,
    #[serde(rename = "number")]
    pub number: String,
}

impl ReadListRequestBookDto {
    pub fn new(series: Vec<String>, number: String) -> ReadListRequestBookDto {
        ReadListRequestBookDto {
            series,
            number,
        }
    }
}


