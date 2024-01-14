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
pub struct WpContributorDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<f32>,
    #[serde(rename = "links")]
    pub links: Vec<crate::models::WpLinkDto>,
}

impl WpContributorDto {
    pub fn new(name: String, links: Vec<crate::models::WpLinkDto>) -> WpContributorDto {
        WpContributorDto {
            name,
            position: None,
            links,
        }
    }
}

