# \SeriesControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze1**](SeriesControllerApi.md#analyze1) | **POST** /api/v1/series/{seriesId}/analyze | 
[**delete_series**](SeriesControllerApi.md#delete_series) | **DELETE** /api/v1/series/{seriesId}/file | 
[**delete_user_uploaded_series_thumbnail**](SeriesControllerApi.md#delete_user_uploaded_series_thumbnail) | **DELETE** /api/v1/series/{seriesId}/thumbnails/{thumbnailId} | 
[**get_all_books_by_series**](SeriesControllerApi.md#get_all_books_by_series) | **GET** /api/v1/series/{seriesId}/books | 
[**get_all_collections_by_series**](SeriesControllerApi.md#get_all_collections_by_series) | **GET** /api/v1/series/{seriesId}/collections | 
[**get_all_series**](SeriesControllerApi.md#get_all_series) | **GET** /api/v1/series | 
[**get_alphabetical_groups**](SeriesControllerApi.md#get_alphabetical_groups) | **GET** /api/v1/series/alphabetical-groups | 
[**get_latest_series**](SeriesControllerApi.md#get_latest_series) | **GET** /api/v1/series/latest | 
[**get_new_series**](SeriesControllerApi.md#get_new_series) | **GET** /api/v1/series/new | 
[**get_one_series**](SeriesControllerApi.md#get_one_series) | **GET** /api/v1/series/{seriesId} | 
[**get_read_progress_tachiyomi_v2**](SeriesControllerApi.md#get_read_progress_tachiyomi_v2) | **GET** /api/v2/series/{seriesId}/read-progress/tachiyomi | 
[**get_series_default_thumbnail**](SeriesControllerApi.md#get_series_default_thumbnail) | **GET** /api/v1/series/{seriesId}/thumbnail | 
[**get_series_file**](SeriesControllerApi.md#get_series_file) | **GET** /api/v1/series/{seriesId}/file | 
[**get_series_thumbnail_by_id**](SeriesControllerApi.md#get_series_thumbnail_by_id) | **GET** /api/v1/series/{seriesId}/thumbnails/{thumbnailId} | 
[**get_series_thumbnails**](SeriesControllerApi.md#get_series_thumbnails) | **GET** /api/v1/series/{seriesId}/thumbnails | 
[**get_updated_series**](SeriesControllerApi.md#get_updated_series) | **GET** /api/v1/series/updated | 
[**mark_as_read**](SeriesControllerApi.md#mark_as_read) | **POST** /api/v1/series/{seriesId}/read-progress | 
[**mark_as_unread**](SeriesControllerApi.md#mark_as_unread) | **DELETE** /api/v1/series/{seriesId}/read-progress | 
[**mark_read_progress_tachiyomi_v2**](SeriesControllerApi.md#mark_read_progress_tachiyomi_v2) | **PUT** /api/v2/series/{seriesId}/read-progress/tachiyomi | 
[**post_mark_selected_series_thumbnail**](SeriesControllerApi.md#post_mark_selected_series_thumbnail) | **PUT** /api/v1/series/{seriesId}/thumbnails/{thumbnailId}/selected | 
[**post_user_uploaded_series_thumbnail**](SeriesControllerApi.md#post_user_uploaded_series_thumbnail) | **POST** /api/v1/series/{seriesId}/thumbnails | 
[**refresh_metadata**](SeriesControllerApi.md#refresh_metadata) | **POST** /api/v1/series/{seriesId}/metadata/refresh | 
[**update_metadata**](SeriesControllerApi.md#update_metadata) | **PATCH** /api/v1/series/{seriesId}/metadata | 



## analyze1

> analyze1(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_series

> delete_series(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_uploaded_series_thumbnail

> delete_user_uploaded_series_thumbnail(series_id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_books_by_series

> crate::models::PageBookDto get_all_books_by_series(series_id, media_status, read_status, tag, deleted, unpaged, page, size, sort, author)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**media_status** | Option<[**Vec<String>**](String.md)> |  |  |
**read_status** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |
**author** | Option<[**Vec<String>**](String.md)> | Author criteria in the format: name,role. Multiple author criteria are supported. |  |

### Return type

[**crate::models::PageBookDto**](PageBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_collections_by_series

> Vec<crate::models::CollectionDto> get_all_collections_by_series(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::CollectionDto>**](CollectionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_series

> crate::models::PageSeriesDto get_all_series(search, library_id, collection_id, status, read_status, publisher, language, genre, tag, age_rating, release_year, sharing_label, deleted, complete, oneshot, unpaged, search_regex, page, size, sort, author)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**collection_id** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**read_status** | Option<[**Vec<String>**](String.md)> |  |  |
**publisher** | Option<[**Vec<String>**](String.md)> |  |  |
**language** | Option<[**Vec<String>**](String.md)> |  |  |
**genre** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**age_rating** | Option<[**Vec<String>**](String.md)> |  |  |
**release_year** | Option<[**Vec<String>**](String.md)> |  |  |
**sharing_label** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**complete** | Option<**bool**> |  |  |
**oneshot** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**search_regex** | Option<**String**> | Search by regex criteria, in the form: regex,field. Supported fields are TITLE and TITLE_SORT. |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |
**author** | Option<[**Vec<String>**](String.md)> | Author criteria in the format: name,role. Multiple author criteria are supported. |  |

### Return type

[**crate::models::PageSeriesDto**](PageSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alphabetical_groups

> Vec<crate::models::GroupCountDto> get_alphabetical_groups(search, library_id, collection_id, status, read_status, publisher, language, genre, tag, age_rating, release_year, sharing_label, deleted, complete, oneshot, search_regex, author)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**collection_id** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**read_status** | Option<[**Vec<String>**](String.md)> |  |  |
**publisher** | Option<[**Vec<String>**](String.md)> |  |  |
**language** | Option<[**Vec<String>**](String.md)> |  |  |
**genre** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**age_rating** | Option<[**Vec<String>**](String.md)> |  |  |
**release_year** | Option<[**Vec<String>**](String.md)> |  |  |
**sharing_label** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**complete** | Option<**bool**> |  |  |
**oneshot** | Option<**bool**> |  |  |
**search_regex** | Option<**String**> | Search by regex criteria, in the form: regex,field. Supported fields are TITLE and TITLE_SORT. |  |
**author** | Option<[**Vec<String>**](String.md)> | Author criteria in the format: name,role. Multiple author criteria are supported. |  |

### Return type

[**Vec<crate::models::GroupCountDto>**](GroupCountDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_series

> crate::models::PageSeriesDto get_latest_series(library_id, deleted, oneshot, unpaged, page, size)


Return recently added or updated series.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**oneshot** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageSeriesDto**](PageSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_new_series

> crate::models::PageSeriesDto get_new_series(library_id, deleted, oneshot, unpaged, page, size)


Return newly added series.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**oneshot** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageSeriesDto**](PageSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_series

> crate::models::SeriesDto get_one_series(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

[**crate::models::SeriesDto**](SeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_progress_tachiyomi_v2

> crate::models::TachiyomiReadProgressV2Dto get_read_progress_tachiyomi_v2(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

[**crate::models::TachiyomiReadProgressV2Dto**](TachiyomiReadProgressV2Dto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_default_thumbnail

> std::path::PathBuf get_series_default_thumbnail(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_file

> serde_json::Value get_series_file(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_thumbnail_by_id

> std::path::PathBuf get_series_thumbnail_by_id(series_id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_thumbnails

> Vec<crate::models::ThumbnailSeriesDto> get_series_thumbnails(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ThumbnailSeriesDto>**](ThumbnailSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_updated_series

> crate::models::PageSeriesDto get_updated_series(library_id, deleted, oneshot, unpaged, page, size)


Return recently updated series, but not newly added ones.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**oneshot** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageSeriesDto**](PageSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_as_read

> mark_as_read(series_id)


Mark all book for series as read

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_as_unread

> mark_as_unread(series_id)


Mark all book for series as unread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_read_progress_tachiyomi_v2

> mark_read_progress_tachiyomi_v2(series_id, tachiyomi_read_progress_update_v2_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**tachiyomi_read_progress_update_v2_dto** | [**TachiyomiReadProgressUpdateV2Dto**](TachiyomiReadProgressUpdateV2Dto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_mark_selected_series_thumbnail

> post_mark_selected_series_thumbnail(series_id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_uploaded_series_thumbnail

> crate::models::ThumbnailSeriesDto post_user_uploaded_series_thumbnail(series_id, file, selected)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**selected** | Option<**bool**> |  |  |

### Return type

[**crate::models::ThumbnailSeriesDto**](ThumbnailSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_metadata

> refresh_metadata(series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metadata

> update_metadata(series_id, series_metadata_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** |  | [required] |
**series_metadata_update_dto** | [**SeriesMetadataUpdateDto**](SeriesMetadataUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

