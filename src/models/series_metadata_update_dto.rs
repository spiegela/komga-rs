/*
 * Komga API
 *
 * Komga offers 2 APIs: REST and OPDS.  Both APIs are secured using HTTP Basic Authentication.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SeriesMetadataUpdateDto : Metadata fields to update. Set a field to null to unset the metadata. You can omit fields you don't want to update.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesMetadataUpdateDto {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "statusLock", skip_serializing_if = "Option::is_none")]
    pub status_lock: Option<bool>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "titleLock", skip_serializing_if = "Option::is_none")]
    pub title_lock: Option<bool>,
    #[serde(rename = "titleSort", skip_serializing_if = "Option::is_none")]
    pub title_sort: Option<String>,
    #[serde(rename = "titleSortLock", skip_serializing_if = "Option::is_none")]
    pub title_sort_lock: Option<bool>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "summaryLock", skip_serializing_if = "Option::is_none")]
    pub summary_lock: Option<bool>,
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "publisherLock", skip_serializing_if = "Option::is_none")]
    pub publisher_lock: Option<bool>,
    #[serde(rename = "readingDirectionLock", skip_serializing_if = "Option::is_none")]
    pub reading_direction_lock: Option<bool>,
    #[serde(rename = "ageRatingLock", skip_serializing_if = "Option::is_none")]
    pub age_rating_lock: Option<bool>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "languageLock", skip_serializing_if = "Option::is_none")]
    pub language_lock: Option<bool>,
    #[serde(rename = "genresLock", skip_serializing_if = "Option::is_none")]
    pub genres_lock: Option<bool>,
    #[serde(rename = "tagsLock", skip_serializing_if = "Option::is_none")]
    pub tags_lock: Option<bool>,
    #[serde(rename = "totalBookCountLock", skip_serializing_if = "Option::is_none")]
    pub total_book_count_lock: Option<bool>,
    #[serde(rename = "sharingLabelsLock", skip_serializing_if = "Option::is_none")]
    pub sharing_labels_lock: Option<bool>,
    #[serde(rename = "linksLock", skip_serializing_if = "Option::is_none")]
    pub links_lock: Option<bool>,
    #[serde(rename = "alternateTitlesLock", skip_serializing_if = "Option::is_none")]
    pub alternate_titles_lock: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::WebLinkUpdateDto>>,
    #[serde(rename = "readingDirection", skip_serializing_if = "Option::is_none")]
    pub reading_direction: Option<ReadingDirection>,
    #[serde(rename = "ageRating", skip_serializing_if = "Option::is_none")]
    pub age_rating: Option<i32>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "totalBookCount", skip_serializing_if = "Option::is_none")]
    pub total_book_count: Option<i32>,
    #[serde(rename = "sharingLabels", skip_serializing_if = "Option::is_none")]
    pub sharing_labels: Option<Vec<String>>,
    #[serde(rename = "alternateTitles", skip_serializing_if = "Option::is_none")]
    pub alternate_titles: Option<Vec<crate::models::AlternateTitleUpdateDto>>,
}

impl SeriesMetadataUpdateDto {
    /// Metadata fields to update. Set a field to null to unset the metadata. You can omit fields you don't want to update.
    pub fn new() -> SeriesMetadataUpdateDto {
        SeriesMetadataUpdateDto {
            status: None,
            status_lock: None,
            title: None,
            title_lock: None,
            title_sort: None,
            title_sort_lock: None,
            summary: None,
            summary_lock: None,
            publisher: None,
            publisher_lock: None,
            reading_direction_lock: None,
            age_rating_lock: None,
            language: None,
            language_lock: None,
            genres_lock: None,
            tags_lock: None,
            total_book_count_lock: None,
            sharing_labels_lock: None,
            links_lock: None,
            alternate_titles_lock: None,
            tags: None,
            links: None,
            reading_direction: None,
            age_rating: None,
            genres: None,
            total_book_count: None,
            sharing_labels: None,
            alternate_titles: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ENDED")]
    Ended,
    #[serde(rename = "ONGOING")]
    Ongoing,
    #[serde(rename = "ABANDONED")]
    Abandoned,
    #[serde(rename = "HIATUS")]
    Hiatus,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ended
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReadingDirection {
    #[serde(rename = "LEFT_TO_RIGHT")]
    LeftToRight,
    #[serde(rename = "RIGHT_TO_LEFT")]
    RightToLeft,
    #[serde(rename = "VERTICAL")]
    Vertical,
    #[serde(rename = "WEBTOON")]
    Webtoon,
}

impl Default for ReadingDirection {
    fn default() -> ReadingDirection {
        Self::LeftToRight
    }
}

