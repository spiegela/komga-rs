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
pub struct WpMetadataDto {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "conformsTo", skip_serializing_if = "Option::is_none")]
    pub conforms_to: Option<String>,
    #[serde(rename = "sortAs", skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<String>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "author")]
    pub author: Vec<String>,
    #[serde(rename = "translator")]
    pub translator: Vec<String>,
    #[serde(rename = "editor")]
    pub editor: Vec<String>,
    #[serde(rename = "artist")]
    pub artist: Vec<String>,
    #[serde(rename = "illustrator")]
    pub illustrator: Vec<String>,
    #[serde(rename = "letterer")]
    pub letterer: Vec<String>,
    #[serde(rename = "penciler")]
    pub penciler: Vec<String>,
    #[serde(rename = "colorist")]
    pub colorist: Vec<String>,
    #[serde(rename = "inker")]
    pub inker: Vec<String>,
    #[serde(rename = "contributor")]
    pub contributor: Vec<String>,
    #[serde(rename = "publisher")]
    pub publisher: Vec<String>,
    #[serde(rename = "subject")]
    pub subject: Vec<String>,
    #[serde(rename = "readingProgression", skip_serializing_if = "Option::is_none")]
    pub reading_progression: Option<ReadingProgression>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "numberOfPages", skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<i32>,
    #[serde(rename = "belongsTo", skip_serializing_if = "Option::is_none")]
    pub belongs_to: Option<Box<crate::models::WpBelongsToDto>>,
    #[serde(rename = "rendition")]
    pub rendition: ::std::collections::HashMap<String, serde_json::Value>,
}

impl WpMetadataDto {
    pub fn new(title: String, author: Vec<String>, translator: Vec<String>, editor: Vec<String>, artist: Vec<String>, illustrator: Vec<String>, letterer: Vec<String>, penciler: Vec<String>, colorist: Vec<String>, inker: Vec<String>, contributor: Vec<String>, publisher: Vec<String>, subject: Vec<String>, rendition: ::std::collections::HashMap<String, serde_json::Value>) -> WpMetadataDto {
        WpMetadataDto {
            title,
            identifier: None,
            r#type: None,
            conforms_to: None,
            sort_as: None,
            subtitle: None,
            modified: None,
            published: None,
            language: None,
            author,
            translator,
            editor,
            artist,
            illustrator,
            letterer,
            penciler,
            colorist,
            inker,
            contributor,
            publisher,
            subject,
            reading_progression: None,
            description: None,
            number_of_pages: None,
            belongs_to: None,
            rendition,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReadingProgression {
    #[serde(rename = "rtl")]
    Rtl,
    #[serde(rename = "ltr")]
    Ltr,
    #[serde(rename = "ttb")]
    Ttb,
    #[serde(rename = "btt")]
    Btt,
    #[serde(rename = "auto")]
    Auto,
}

impl Default for ReadingProgression {
    fn default() -> ReadingProgression {
        Self::Rtl
    }
}

