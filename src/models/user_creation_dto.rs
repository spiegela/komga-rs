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
pub struct UserCreationDto {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
}

impl UserCreationDto {
    pub fn new(email: String, password: String, roles: Vec<String>) -> UserCreationDto {
        UserCreationDto {
            email,
            password,
            roles,
        }
    }
}


