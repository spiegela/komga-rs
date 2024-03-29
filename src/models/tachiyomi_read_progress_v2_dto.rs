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
pub struct TachiyomiReadProgressV2Dto {
    #[serde(rename = "booksCount")]
    pub books_count: i32,
    #[serde(rename = "booksReadCount")]
    pub books_read_count: i32,
    #[serde(rename = "booksUnreadCount")]
    pub books_unread_count: i32,
    #[serde(rename = "booksInProgressCount")]
    pub books_in_progress_count: i32,
    #[serde(rename = "lastReadContinuousNumberSort")]
    pub last_read_continuous_number_sort: f32,
    #[serde(rename = "maxNumberSort")]
    pub max_number_sort: f32,
}

impl TachiyomiReadProgressV2Dto {
    pub fn new(books_count: i32, books_read_count: i32, books_unread_count: i32, books_in_progress_count: i32, last_read_continuous_number_sort: f32, max_number_sort: f32) -> TachiyomiReadProgressV2Dto {
        TachiyomiReadProgressV2Dto {
            books_count,
            books_read_count,
            books_unread_count,
            books_in_progress_count,
            last_read_continuous_number_sort,
            max_number_sort,
        }
    }
}


