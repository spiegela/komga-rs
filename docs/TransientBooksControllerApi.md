# \TransientBooksControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze**](TransientBooksControllerApi.md#analyze) | **POST** /api/v1/transient-books/{id}/analyze | 
[**get_source_page**](TransientBooksControllerApi.md#get_source_page) | **GET** /api/v1/transient-books/{id}/pages/{pageNumber} | 
[**scan_for_transient_books**](TransientBooksControllerApi.md#scan_for_transient_books) | **POST** /api/v1/transient-books | 



## analyze

> crate::models::TransientBookDto analyze(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::TransientBookDto**](TransientBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source_page

> String get_source_page(id, page_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**page_number** | **i32** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_for_transient_books

> Vec<crate::models::TransientBookDto> scan_for_transient_books(scan_request_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_request_dto** | [**ScanRequestDto**](ScanRequestDto.md) |  | [required] |

### Return type

[**Vec<crate::models::TransientBookDto>**](TransientBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

