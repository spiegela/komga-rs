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
pub struct ReadListRequestBookMatchBookDto {
    #[serde(rename = "bookId")]
    pub book_id: String,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl ReadListRequestBookMatchBookDto {
    pub fn new(book_id: String, number: String, title: String) -> ReadListRequestBookMatchBookDto {
        ReadListRequestBookMatchBookDto {
            book_id,
            number,
            title,
        }
    }
}

