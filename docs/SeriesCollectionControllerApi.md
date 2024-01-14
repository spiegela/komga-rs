# \SeriesCollectionControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_one3**](SeriesCollectionControllerApi.md#add_one3) | **POST** /api/v1/collections | 
[**add_user_uploaded_collection_thumbnail**](SeriesCollectionControllerApi.md#add_user_uploaded_collection_thumbnail) | **POST** /api/v1/collections/{id}/thumbnails | 
[**delete_one2**](SeriesCollectionControllerApi.md#delete_one2) | **DELETE** /api/v1/collections/{id} | 
[**delete_user_uploaded_collection_thumbnail**](SeriesCollectionControllerApi.md#delete_user_uploaded_collection_thumbnail) | **DELETE** /api/v1/collections/{id}/thumbnails/{thumbnailId} | 
[**get_all4**](SeriesCollectionControllerApi.md#get_all4) | **GET** /api/v1/collections | 
[**get_collection_thumbnail**](SeriesCollectionControllerApi.md#get_collection_thumbnail) | **GET** /api/v1/collections/{id}/thumbnail | 
[**get_collection_thumbnail_by_id**](SeriesCollectionControllerApi.md#get_collection_thumbnail_by_id) | **GET** /api/v1/collections/{id}/thumbnails/{thumbnailId} | 
[**get_collection_thumbnails**](SeriesCollectionControllerApi.md#get_collection_thumbnails) | **GET** /api/v1/collections/{id}/thumbnails | 
[**get_one2**](SeriesCollectionControllerApi.md#get_one2) | **GET** /api/v1/collections/{id} | 
[**get_series_for_collection**](SeriesCollectionControllerApi.md#get_series_for_collection) | **GET** /api/v1/collections/{id}/series | 
[**mark_selected_collection_thumbnail**](SeriesCollectionControllerApi.md#mark_selected_collection_thumbnail) | **PUT** /api/v1/collections/{id}/thumbnails/{thumbnailId}/selected | 
[**update_one2**](SeriesCollectionControllerApi.md#update_one2) | **PATCH** /api/v1/collections/{id} | 



## add_one3

> crate::models::CollectionDto add_one3(collection_creation_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_creation_dto** | [**CollectionCreationDto**](CollectionCreationDto.md) |  | [required] |

### Return type

[**crate::models::CollectionDto**](CollectionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_uploaded_collection_thumbnail

> crate::models::ThumbnailSeriesCollectionDto add_user_uploaded_collection_thumbnail(id, file, selected)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**selected** | Option<**bool**> |  |  |

### Return type

[**crate::models::ThumbnailSeriesCollectionDto**](ThumbnailSeriesCollectionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_one2

> delete_one2(id)


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


## delete_user_uploaded_collection_thumbnail

> delete_user_uploaded_collection_thumbnail(id, thumbnail_id)


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


## get_all4

> crate::models::PageCollectionDto get_all4(search, library_id, unpaged, page, size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageCollectionDto**](PageCollectionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_thumbnail

> std::path::PathBuf get_collection_thumbnail(id)


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


## get_collection_thumbnail_by_id

> std::path::PathBuf get_collection_thumbnail_by_id(id, thumbnail_id)


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


## get_collection_thumbnails

> Vec<crate::models::ThumbnailSeriesCollectionDto> get_collection_thumbnails(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ThumbnailSeriesCollectionDto>**](ThumbnailSeriesCollectionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one2

> crate::models::CollectionDto get_one2(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CollectionDto**](CollectionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_for_collection

> crate::models::PageSeriesDto get_series_for_collection(id, library_id, status, read_status, publisher, language, genre, tag, age_rating, release_years, deleted, complete, unpaged, page, size, author)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**read_status** | Option<[**Vec<String>**](String.md)> |  |  |
**publisher** | Option<[**Vec<String>**](String.md)> |  |  |
**language** | Option<[**Vec<String>**](String.md)> |  |  |
**genre** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**age_rating** | Option<[**Vec<String>**](String.md)> |  |  |
**release_years** | Option<[**Vec<String>**](String.md)> |  |  |
**deleted** | Option<**bool**> |  |  |
**complete** | Option<**bool**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**author** | Option<[**Vec<String>**](String.md)> | Author criteria in the format: name,role. Multiple author criteria are supported. |  |

### Return type

[**crate::models::PageSeriesDto**](PageSeriesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_selected_collection_thumbnail

> mark_selected_collection_thumbnail(id, thumbnail_id)


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


## update_one2

> update_one2(id, collection_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**collection_update_dto** | [**CollectionUpdateDto**](CollectionUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

