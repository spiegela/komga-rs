# \HistoricalEventControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all3**](HistoricalEventControllerApi.md#get_all3) | **GET** /api/v1/history | 



## get_all3

> crate::models::PageHistoricalEventDto get_all3(page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PageHistoricalEventDto**](PageHistoricalEventDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

