# \AnnouncementControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_announcements**](AnnouncementControllerApi.md#get_announcements) | **GET** /api/v1/announcements | 
[**mark_announcements_read**](AnnouncementControllerApi.md#mark_announcements_read) | **PUT** /api/v1/announcements | 



## get_announcements

> crate::models::JsonFeedDto get_announcements()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::JsonFeedDto**](JsonFeedDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_announcements_read

> mark_announcements_read(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

