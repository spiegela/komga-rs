# \ReferentialControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_age_ratings**](ReferentialControllerApi.md#get_age_ratings) | **GET** /api/v1/age-ratings | 
[**get_authors**](ReferentialControllerApi.md#get_authors) | **GET** /api/v2/authors | 
[**get_authors_names**](ReferentialControllerApi.md#get_authors_names) | **GET** /api/v1/authors/names | 
[**get_authors_roles**](ReferentialControllerApi.md#get_authors_roles) | **GET** /api/v1/authors/roles | 
[**get_authors_v1**](ReferentialControllerApi.md#get_authors_v1) | **GET** /api/v1/authors | 
[**get_book_tags**](ReferentialControllerApi.md#get_book_tags) | **GET** /api/v1/tags/book | 
[**get_genres**](ReferentialControllerApi.md#get_genres) | **GET** /api/v1/genres | 
[**get_languages**](ReferentialControllerApi.md#get_languages) | **GET** /api/v1/languages | 
[**get_publishers**](ReferentialControllerApi.md#get_publishers) | **GET** /api/v1/publishers | 
[**get_series_release_dates**](ReferentialControllerApi.md#get_series_release_dates) | **GET** /api/v1/series/release-dates | 
[**get_series_tags**](ReferentialControllerApi.md#get_series_tags) | **GET** /api/v1/tags/series | 
[**get_sharing_labels**](ReferentialControllerApi.md#get_sharing_labels) | **GET** /api/v1/sharing-labels | 
[**get_tags**](ReferentialControllerApi.md#get_tags) | **GET** /api/v1/tags | 



## get_age_ratings

> Vec<String> get_age_ratings(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authors

> crate::models::PageAuthorDto get_authors(search, role, library_id, collection_id, series_id, readlist_id, unpaged, page, size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |
**series_id** | Option<**String**> |  |  |
**readlist_id** | Option<**String**> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageAuthorDto**](PageAuthorDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authors_names

> Vec<String> get_authors_names(search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |[default to ]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authors_roles

> Vec<String> get_authors_roles()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authors_v1

> Vec<crate::models::AuthorDto> get_authors_v1(search, library_id, collection_id, series_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |[default to ]
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |
**series_id** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AuthorDto>**](AuthorDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_tags

> Vec<String> get_book_tags(series_id, readlist_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**String**> |  |  |
**readlist_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genres

> Vec<String> get_genres(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages

> Vec<String> get_languages(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_publishers

> Vec<String> get_publishers(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_release_dates

> Vec<String> get_series_release_dates(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_tags

> Vec<String> get_series_tags(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sharing_labels

> Vec<String> get_sharing_labels(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> Vec<String> get_tags(library_id, collection_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<**String**> |  |  |
**collection_id** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

