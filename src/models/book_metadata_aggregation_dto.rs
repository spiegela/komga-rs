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
pub struct BookMetadataAggregationDto {
    #[serde(rename = "authors")]
    pub authors: Vec<crate::models::AuthorDto>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "summaryNumber")]
    pub summary_number: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
}

impl BookMetadataAggregationDto {
    pub fn new(authors: Vec<crate::models::AuthorDto>, tags: Vec<String>, summary: String, summary_number: String, created: String, last_modified: String) -> BookMetadataAggregationDto {
        BookMetadataAggregationDto {
            authors,
            tags,
            release_date: None,
            summary,
            summary_number,
            created,
            last_modified,
        }
    }
}


