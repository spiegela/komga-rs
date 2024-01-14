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
pub struct ThumbnailSeriesDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "seriesId")]
    pub series_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "selected")]
    pub selected: bool,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    #[serde(rename = "width")]
    pub width: i32,
    #[serde(rename = "height")]
    pub height: i32,
}

impl ThumbnailSeriesDto {
    pub fn new(id: String, series_id: String, r#type: String, selected: bool, media_type: String, file_size: i64, width: i32, height: i32) -> ThumbnailSeriesDto {
        ThumbnailSeriesDto {
            id,
            series_id,
            r#type,
            selected,
            media_type,
            file_size,
            width,
            height,
        }
    }
}

