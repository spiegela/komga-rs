# \LibraryControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_one2**](LibraryControllerApi.md#add_one2) | **POST** /api/v1/libraries | 
[**analyze2**](LibraryControllerApi.md#analyze2) | **POST** /api/v1/libraries/{libraryId}/analyze | 
[**delete_one1**](LibraryControllerApi.md#delete_one1) | **DELETE** /api/v1/libraries/{libraryId} | 
[**empty_trash**](LibraryControllerApi.md#empty_trash) | **POST** /api/v1/libraries/{libraryId}/empty-trash | 
[**get_all2**](LibraryControllerApi.md#get_all2) | **GET** /api/v1/libraries | 
[**get_one1**](LibraryControllerApi.md#get_one1) | **GET** /api/v1/libraries/{libraryId} | 
[**patch_one**](LibraryControllerApi.md#patch_one) | **PATCH** /api/v1/libraries/{libraryId} | 
[**refresh_metadata1**](LibraryControllerApi.md#refresh_metadata1) | **POST** /api/v1/libraries/{libraryId}/metadata/refresh | 
[**scan**](LibraryControllerApi.md#scan) | **POST** /api/v1/libraries/{libraryId}/scan | 
[**update_one**](LibraryControllerApi.md#update_one) | **PUT** /api/v1/libraries/{libraryId} | 



## add_one2

> crate::models::LibraryDto add_one2(library_creation_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_creation_dto** | [**LibraryCreationDto**](LibraryCreationDto.md) |  | [required] |

### Return type

[**crate::models::LibraryDto**](LibraryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analyze2

> analyze2(library_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_one1

> delete_one1(library_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## empty_trash

> empty_trash(library_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all2

> Vec<crate::models::LibraryDto> get_all2()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LibraryDto>**](LibraryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one1

> crate::models::LibraryDto get_one1(library_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |

### Return type

[**crate::models::LibraryDto**](LibraryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_one

> patch_one(library_id, library_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |
**library_update_dto** | [**LibraryUpdateDto**](LibraryUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_metadata1

> refresh_metadata1(library_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan

> scan(library_id, deep)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |
**deep** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_one

> update_one(library_id, library_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** |  | [required] |
**library_update_dto** | [**LibraryUpdateDto**](LibraryUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

