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
pub struct R2Positions {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "positions")]
    pub positions: Vec<crate::models::R2Locator>,
}

impl R2Positions {
    pub fn new(total: i32, positions: Vec<crate::models::R2Locator>) -> R2Positions {
        R2Positions {
            total,
            positions,
        }
    }
}


