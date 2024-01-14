# \UserControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_one**](UserControllerApi.md#add_one) | **POST** /api/v2/users | 
[**delete**](UserControllerApi.md#delete) | **DELETE** /api/v2/users/{id} | 
[**get_all**](UserControllerApi.md#get_all) | **GET** /api/v2/users | 
[**get_authentication_activity**](UserControllerApi.md#get_authentication_activity) | **GET** /api/v2/users/authentication-activity | 
[**get_latest_authentication_activity_for_user**](UserControllerApi.md#get_latest_authentication_activity_for_user) | **GET** /api/v2/users/{id}/authentication-activity/latest | 
[**get_me**](UserControllerApi.md#get_me) | **GET** /api/v2/users/me | 
[**get_my_authentication_activity**](UserControllerApi.md#get_my_authentication_activity) | **GET** /api/v2/users/me/authentication-activity | 
[**update_my_password**](UserControllerApi.md#update_my_password) | **PATCH** /api/v2/users/me/password | 
[**update_password**](UserControllerApi.md#update_password) | **PATCH** /api/v2/users/{id}/password | 
[**update_user**](UserControllerApi.md#update_user) | **PATCH** /api/v2/users/{id} | 



## add_one

> crate::models::UserDto add_one(user_creation_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_creation_dto** | [**UserCreationDto**](UserCreationDto.md) |  | [required] |

### Return type

[**crate::models::UserDto**](UserDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete

> delete(id)


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


## get_all

> Vec<crate::models::UserDto> get_all()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserDto>**](UserDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authentication_activity

> crate::models::PageAuthenticationActivityDto get_authentication_activity(unpaged, page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |[default to 0]
**size** | Option<**i32**> | The size of the page to be returned |  |[default to 20]
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property,(asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PageAuthenticationActivityDto**](PageAuthenticationActivityDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_authentication_activity_for_user

> crate::models::AuthenticationActivityDto get_latest_authentication_activity_for_user(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::AuthenticationActivityDto**](AuthenticationActivityDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me

> crate::models::UserDto get_me()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserDto**](UserDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_authentication_activity

> crate::models::PageAuthenticationActivityDto get_my_authentication_activity(unpaged, page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |[default to 0]
**size** | Option<**i32**> | The size of the page to be returned |  |[default to 20]
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property,(asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PageAuthenticationActivityDto**](PageAuthenticationActivityDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_password

> update_my_password(password_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_update_dto** | [**PasswordUpdateDto**](PasswordUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_password

> update_password(id, password_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**password_update_dto** | [**PasswordUpdateDto**](PasswordUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(id, user_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_update_dto** | [**UserUpdateDto**](UserUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

