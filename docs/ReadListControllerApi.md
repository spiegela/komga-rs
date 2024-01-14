# \ReadListControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_one1**](ReadListControllerApi.md#add_one1) | **POST** /api/v1/readlists | 
[**add_user_uploaded_read_list_thumbnail**](ReadListControllerApi.md#add_user_uploaded_read_list_thumbnail) | **POST** /api/v1/readlists/{id}/thumbnails | 
[**delete_one**](ReadListControllerApi.md#delete_one) | **DELETE** /api/v1/readlists/{id} | 
[**delete_user_uploaded_read_list_thumbnail**](ReadListControllerApi.md#delete_user_uploaded_read_list_thumbnail) | **DELETE** /api/v1/readlists/{id}/thumbnails/{thumbnailId} | 
[**get_all1**](ReadListControllerApi.md#get_all1) | **GET** /api/v1/readlists | 
[**get_book_sibling_next**](ReadListControllerApi.md#get_book_sibling_next) | **GET** /api/v1/readlists/{id}/books/{bookId}/next | 
[**get_book_sibling_previous**](ReadListControllerApi.md#get_book_sibling_previous) | **GET** /api/v1/readlists/{id}/books/{bookId}/previous | 
[**get_books_for_read_list**](ReadListControllerApi.md#get_books_for_read_list) | **GET** /api/v1/readlists/{id}/books | 
[**get_one**](ReadListControllerApi.md#get_one) | **GET** /api/v1/readlists/{id} | 
[**get_read_list_file**](ReadListControllerApi.md#get_read_list_file) | **GET** /api/v1/readlists/{id}/file | 
[**get_read_list_thumbnail**](ReadListControllerApi.md#get_read_list_thumbnail) | **GET** /api/v1/readlists/{id}/thumbnail | 
[**get_read_list_thumbnail_by_id**](ReadListControllerApi.md#get_read_list_thumbnail_by_id) | **GET** /api/v1/readlists/{id}/thumbnails/{thumbnailId} | 
[**get_read_list_thumbnails**](ReadListControllerApi.md#get_read_list_thumbnails) | **GET** /api/v1/readlists/{id}/thumbnails | 
[**get_read_progress**](ReadListControllerApi.md#get_read_progress) | **GET** /api/v1/readlists/{id}/read-progress/tachiyomi | 
[**mark_read_progress_tachiyomi**](ReadListControllerApi.md#mark_read_progress_tachiyomi) | **PUT** /api/v1/readlists/{id}/read-progress/tachiyomi | 
[**mark_selected_read_list_thumbnail**](ReadListControllerApi.md#mark_selected_read_list_thumbnail) | **PUT** /api/v1/readlists/{id}/thumbnails/{thumbnailId}/selected | 
[**match_from_comic_rack_list**](ReadListControllerApi.md#match_from_comic_rack_list) | **POST** /api/v1/readlists/match/comicrack | 
[**update_one1**](ReadListControllerApi.md#update_one1) | **PATCH** /api/v1/readlists/{id} | 



## add_one1

> crate::models::ReadListDto add_one1(read_list_creation_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_list_creation_dto** | [**ReadListCreationDto**](ReadListCreationDto.md) |  | [required] |

### Return type

[**crate::models::ReadListDto**](ReadListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_uploaded_read_list_thumbnail

> crate::models::ThumbnailReadListDto add_user_uploaded_read_list_thumbnail(id, file, selected)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**selected** | Option<**bool**> |  |  |

### Return type

[**crate::models::ThumbnailReadListDto**](ThumbnailReadListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_one

> delete_one(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_uploaded_read_list_thumbnail

> delete_user_uploaded_read_list_thumbnail(id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all1

> crate::models::PageReadListDto get_all1(search, library_id, unpaged, page, size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageReadListDto**](PageReadListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_sibling_next

> crate::models::BookDto get_book_sibling_next(id, book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::BookDto**](BookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_sibling_previous

> crate::models::BookDto get_book_sibling_previous(id, book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::BookDto**](BookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_books_for_read_list

> crate::models::PageBookDto get_books_for_read_list(id, library_id, read_status, tag, media_status, deleted, unpaged, page, size, author)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**read_status** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**media_status** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**author** | Option<[**Vec<String>**](String.md)> | Author criteria in the format: name,role. Multiple author criteria are supported. |  |

### Return type

[**crate::models::PageBookDto**](PageBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one

> crate::models::ReadListDto get_one(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ReadListDto**](ReadListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_list_file

> serde_json::Value get_read_list_file(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_list_thumbnail

> std::path::PathBuf get_read_list_thumbnail(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_list_thumbnail_by_id

> std::path::PathBuf get_read_list_thumbnail_by_id(id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_list_thumbnails

> Vec<crate::models::ThumbnailReadListDto> get_read_list_thumbnails(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ThumbnailReadListDto>**](ThumbnailReadListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_read_progress

> crate::models::TachiyomiReadProgressDto get_read_progress(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::TachiyomiReadProgressDto**](TachiyomiReadProgressDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_read_progress_tachiyomi

> mark_read_progress_tachiyomi(id, tachiyomi_read_progress_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tachiyomi_read_progress_update_dto** | [**TachiyomiReadProgressUpdateDto**](TachiyomiReadProgressUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_selected_read_list_thumbnail

> mark_selected_read_list_thumbnail(id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## match_from_comic_rack_list

> crate::models::ReadListRequestMatchDto match_from_comic_rack_list(post_user_uploaded_series_thumbnail_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_user_uploaded_series_thumbnail_request** | Option<[**PostUserUploadedSeriesThumbnailRequest**](PostUserUploadedSeriesThumbnailRequest.md)> |  |  |

### Return type

[**crate::models::ReadListRequestMatchDto**](ReadListRequestMatchDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_one1

> update_one1(id, read_list_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**read_list_update_dto** | [**ReadListUpdateDto**](ReadListUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

