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
pub struct OAuth2ClientDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "registrationId")]
    pub registration_id: String,
}

impl OAuth2ClientDto {
    pub fn new(name: String, registration_id: String) -> OAuth2ClientDto {
        OAuth2ClientDto {
            name,
            registration_id,
        }
    }
}

