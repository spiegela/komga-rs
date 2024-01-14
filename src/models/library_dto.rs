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
pub struct LibraryDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "root")]
    pub root: String,
    #[serde(rename = "importComicInfoBook")]
    pub import_comic_info_book: bool,
    #[serde(rename = "importComicInfoSeries")]
    pub import_comic_info_series: bool,
    #[serde(rename = "importComicInfoCollection")]
    pub import_comic_info_collection: bool,
    #[serde(rename = "importComicInfoReadList")]
    pub import_comic_info_read_list: bool,
    #[serde(rename = "importComicInfoSeriesAppendVolume")]
    pub import_comic_info_series_append_volume: bool,
    #[serde(rename = "importEpubBook")]
    pub import_epub_book: bool,
    #[serde(rename = "importEpubSeries")]
    pub import_epub_series: bool,
    #[serde(rename = "importMylarSeries")]
    pub import_mylar_series: bool,
    #[serde(rename = "importLocalArtwork")]
    pub import_local_artwork: bool,
    #[serde(rename = "importBarcodeIsbn")]
    pub import_barcode_isbn: bool,
    #[serde(rename = "scanForceModifiedTime")]
    pub scan_force_modified_time: bool,
    #[serde(rename = "scanInterval")]
    pub scan_interval: ScanInterval,
    #[serde(rename = "scanOnStartup")]
    pub scan_on_startup: bool,
    #[serde(rename = "scanCbx")]
    pub scan_cbx: bool,
    #[serde(rename = "scanPdf")]
    pub scan_pdf: bool,
    #[serde(rename = "scanEpub")]
    pub scan_epub: bool,
    #[serde(rename = "scanDirectoryExclusions")]
    pub scan_directory_exclusions: Vec<String>,
    #[serde(rename = "repairExtensions")]
    pub repair_extensions: bool,
    #[serde(rename = "convertToCbz")]
    pub convert_to_cbz: bool,
    #[serde(rename = "emptyTrashAfterScan")]
    pub empty_trash_after_scan: bool,
    #[serde(rename = "seriesCover")]
    pub series_cover: SeriesCover,
    #[serde(rename = "hashFiles")]
    pub hash_files: bool,
    #[serde(rename = "hashPages")]
    pub hash_pages: bool,
    #[serde(rename = "analyzeDimensions")]
    pub analyze_dimensions: bool,
    #[serde(rename = "oneshotsDirectory", skip_serializing_if = "Option::is_none")]
    pub oneshots_directory: Option<String>,
    #[serde(rename = "unavailable")]
    pub unavailable: bool,
}

impl LibraryDto {
    pub fn new(id: String, name: String, root: String, import_comic_info_book: bool, import_comic_info_series: bool, import_comic_info_collection: bool, import_comic_info_read_list: bool, import_comic_info_series_append_volume: bool, import_epub_book: bool, import_epub_series: bool, import_mylar_series: bool, import_local_artwork: bool, import_barcode_isbn: bool, scan_force_modified_time: bool, scan_interval: ScanInterval, scan_on_startup: bool, scan_cbx: bool, scan_pdf: bool, scan_epub: bool, scan_directory_exclusions: Vec<String>, repair_extensions: bool, convert_to_cbz: bool, empty_trash_after_scan: bool, series_cover: SeriesCover, hash_files: bool, hash_pages: bool, analyze_dimensions: bool, unavailable: bool) -> LibraryDto {
        LibraryDto {
            id,
            name,
            root,
            import_comic_info_book,
            import_comic_info_series,
            import_comic_info_collection,
            import_comic_info_read_list,
            import_comic_info_series_append_volume,
            import_epub_book,
            import_epub_series,
            import_mylar_series,
            import_local_artwork,
            import_barcode_isbn,
            scan_force_modified_time,
            scan_interval,
            scan_on_startup,
            scan_cbx,
            scan_pdf,
            scan_epub,
            scan_directory_exclusions,
            repair_extensions,
            convert_to_cbz,
            empty_trash_after_scan,
            series_cover,
            hash_files,
            hash_pages,
            analyze_dimensions,
            oneshots_directory: None,
            unavailable,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScanInterval {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "HOURLY")]
    Hourly,
    #[serde(rename = "EVERY_6H")]
    Every6H,
    #[serde(rename = "EVERY_12H")]
    Every12H,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "WEEKLY")]
    Weekly,
}

impl Default for ScanInterval {
    fn default() -> ScanInterval {
        Self::Disabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeriesCover {
    #[serde(rename = "FIRST")]
    First,
    #[serde(rename = "FIRST_UNREAD_OR_FIRST")]
    FirstUnreadOrFirst,
    #[serde(rename = "FIRST_UNREAD_OR_LAST")]
    FirstUnreadOrLast,
    #[serde(rename = "LAST")]
    Last,
}

impl Default for SeriesCover {
    fn default() -> SeriesCover {
        Self::First
    }
}

