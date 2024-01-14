# \FileSystemControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_directory_listing**](FileSystemControllerApi.md#get_directory_listing) | **POST** /api/v1/filesystem | 



## get_directory_listing

> crate::models::DirectoryListingDto get_directory_listing(directory_request_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**directory_request_dto** | Option<[**DirectoryRequestDto**](DirectoryRequestDto.md)> |  |  |

### Return type

[**crate::models::DirectoryListingDto**](DirectoryListingDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

