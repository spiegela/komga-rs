# \SettingsControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_settings**](SettingsControllerApi.md#get_settings) | **GET** /api/v1/settings | 
[**update_settings**](SettingsControllerApi.md#update_settings) | **PATCH** /api/v1/settings | 



## get_settings

> crate::models::SettingsDto get_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SettingsDto**](SettingsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_settings

> update_settings(settings_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_update_dto** | [**SettingsUpdateDto**](SettingsUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

