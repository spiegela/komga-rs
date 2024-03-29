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
pub struct UserDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "sharedAllLibraries")]
    pub shared_all_libraries: bool,
    #[serde(rename = "sharedLibrariesIds")]
    pub shared_libraries_ids: Vec<String>,
    #[serde(rename = "labelsAllow")]
    pub labels_allow: Vec<String>,
    #[serde(rename = "labelsExclude")]
    pub labels_exclude: Vec<String>,
    #[serde(rename = "ageRestriction", skip_serializing_if = "Option::is_none")]
    pub age_restriction: Option<Box<crate::models::AgeRestrictionDto>>,
}

impl UserDto {
    pub fn new(id: String, email: String, roles: Vec<String>, shared_all_libraries: bool, shared_libraries_ids: Vec<String>, labels_allow: Vec<String>, labels_exclude: Vec<String>) -> UserDto {
        UserDto {
            id,
            email,
            roles,
            shared_all_libraries,
            shared_libraries_ids,
            labels_allow,
            labels_exclude,
            age_restriction: None,
        }
    }
}


