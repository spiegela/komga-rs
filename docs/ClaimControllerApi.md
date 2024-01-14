# \ClaimControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**claim_admin**](ClaimControllerApi.md#claim_admin) | **POST** /api/v1/claim | 
[**get_claim_status**](ClaimControllerApi.md#get_claim_status) | **GET** /api/v1/claim | 



## claim_admin

> crate::models::UserDto claim_admin(x_komga_email, x_komga_password)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_komga_email** | **String** |  | [required] |
**x_komga_password** | **String** |  | [required] |

### Return type

[**crate::models::UserDto**](UserDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_claim_status

> crate::models::ClaimStatus get_claim_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClaimStatus**](ClaimStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

