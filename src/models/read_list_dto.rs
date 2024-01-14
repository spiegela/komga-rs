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
pub struct ReadListDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "ordered")]
    pub ordered: bool,
    #[serde(rename = "bookIds")]
    pub book_ids: Vec<String>,
    #[serde(rename = "createdDate")]
    pub created_date: String,
    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: String,
    #[serde(rename = "filtered")]
    pub filtered: bool,
}

impl ReadListDto {
    pub fn new(id: String, name: String, summary: String, ordered: bool, book_ids: Vec<String>, created_date: String, last_modified_date: String, filtered: bool) -> ReadListDto {
        ReadListDto {
            id,
            name,
            summary,
            ordered,
            book_ids,
            created_date,
            last_modified_date,
            filtered,
        }
    }
}

