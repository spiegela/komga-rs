# \PageHashControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_known_page_hash**](PageHashControllerApi.md#create_or_update_known_page_hash) | **PUT** /api/v1/page-hashes | 
[**delete_single_match**](PageHashControllerApi.md#delete_single_match) | **POST** /api/v1/page-hashes/{pageHash}/delete-match | 
[**get_known_page_hash_thumbnail**](PageHashControllerApi.md#get_known_page_hash_thumbnail) | **GET** /api/v1/page-hashes/{pageHash}/thumbnail | 
[**get_known_page_hashes**](PageHashControllerApi.md#get_known_page_hashes) | **GET** /api/v1/page-hashes | 
[**get_page_hash_matches**](PageHashControllerApi.md#get_page_hash_matches) | **GET** /api/v1/page-hashes/{pageHash} | 
[**get_unknown_page_hash_thumbnail**](PageHashControllerApi.md#get_unknown_page_hash_thumbnail) | **GET** /api/v1/page-hashes/unknown/{pageHash}/thumbnail | 
[**get_unknown_page_hashes**](PageHashControllerApi.md#get_unknown_page_hashes) | **GET** /api/v1/page-hashes/unknown | 
[**perform_delete**](PageHashControllerApi.md#perform_delete) | **POST** /api/v1/page-hashes/{pageHash}/delete-all | 



## create_or_update_known_page_hash

> create_or_update_known_page_hash(page_hash_creation_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_hash_creation_dto** | [**PageHashCreationDto**](PageHashCreationDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_single_match

> delete_single_match(page_hash, page_hash_match_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_hash** | **String** |  | [required] |
**page_hash_match_dto** | [**PageHashMatchDto**](PageHashMatchDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_known_page_hash_thumbnail

> std::path::PathBuf get_known_page_hash_thumbnail(page_hash)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_hash** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_known_page_hashes

> crate::models::PagePageHashKnownDto get_known_page_hashes(action, page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<[**Vec<String>**](String.md)> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PagePageHashKnownDto**](PagePageHashKnownDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_page_hash_matches

> crate::models::PagePageHashMatchDto get_page_hash_matches(page_hash, page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_hash** | **String** |  | [required] |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PagePageHashMatchDto**](PagePageHashMatchDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unknown_page_hash_thumbnail

> std::path::PathBuf get_unknown_page_hash_thumbnail(page_hash, resize)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_hash** | **String** |  | [required] |
**resize** | Option<**i32**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unknown_page_hashes

> crate::models::PagePageHashUnknownDto get_unknown_page_hashes(page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PagePageHashUnknownDto**](PagePageHashUnknownDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## perform_delete

> perform_delete(page_hash)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_hash** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

