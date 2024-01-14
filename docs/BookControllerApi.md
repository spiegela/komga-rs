# \BookControllerApi

All URIs are relative to *http://localhost:25600*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_uploaded_book_thumbnail**](BookControllerApi.md#add_user_uploaded_book_thumbnail) | **POST** /api/v1/books/{bookId}/thumbnails | 
[**analyze3**](BookControllerApi.md#analyze3) | **POST** /api/v1/books/{bookId}/analyze | 
[**delete_book**](BookControllerApi.md#delete_book) | **DELETE** /api/v1/books/{bookId}/file | 
[**delete_read_progress**](BookControllerApi.md#delete_read_progress) | **DELETE** /api/v1/books/{bookId}/read-progress | 
[**delete_user_uploaded_book_thumbnail**](BookControllerApi.md#delete_user_uploaded_book_thumbnail) | **DELETE** /api/v1/books/{bookId}/thumbnails/{thumbnailId} | 
[**get_all_books**](BookControllerApi.md#get_all_books) | **GET** /api/v1/books | 
[**get_all_read_lists_by_book**](BookControllerApi.md#get_all_read_lists_by_book) | **GET** /api/v1/books/{bookId}/readlists | 
[**get_book_file**](BookControllerApi.md#get_book_file) | **GET** /api/v1/books/{bookId}/file | 
[**get_book_file1**](BookControllerApi.md#get_book_file1) | **GET** /api/v1/books/{bookId}/file/* | 
[**get_book_page**](BookControllerApi.md#get_book_page) | **GET** /api/v1/books/{bookId}/pages/{pageNumber} | 
[**get_book_page_raw**](BookControllerApi.md#get_book_page_raw) | **GET** /api/v1/books/{bookId}/pages/{pageNumber}/raw | 
[**get_book_page_thumbnail**](BookControllerApi.md#get_book_page_thumbnail) | **GET** /api/v1/books/{bookId}/pages/{pageNumber}/thumbnail | 
[**get_book_pages**](BookControllerApi.md#get_book_pages) | **GET** /api/v1/books/{bookId}/pages | 
[**get_book_resource**](BookControllerApi.md#get_book_resource) | **GET** /api/v1/books/{bookId}/resource/{resource} | 
[**get_book_sibling_next1**](BookControllerApi.md#get_book_sibling_next1) | **GET** /api/v1/books/{bookId}/next | 
[**get_book_sibling_previous1**](BookControllerApi.md#get_book_sibling_previous1) | **GET** /api/v1/books/{bookId}/previous | 
[**get_book_thumbnail**](BookControllerApi.md#get_book_thumbnail) | **GET** /api/v1/books/{bookId}/thumbnail | 
[**get_book_thumbnail_by_id**](BookControllerApi.md#get_book_thumbnail_by_id) | **GET** /api/v1/books/{bookId}/thumbnails/{thumbnailId} | 
[**get_book_thumbnails**](BookControllerApi.md#get_book_thumbnails) | **GET** /api/v1/books/{bookId}/thumbnails | 
[**get_books_on_deck**](BookControllerApi.md#get_books_on_deck) | **GET** /api/v1/books/ondeck | 
[**get_duplicate_books**](BookControllerApi.md#get_duplicate_books) | **GET** /api/v1/books/duplicates | 
[**get_latest_books**](BookControllerApi.md#get_latest_books) | **GET** /api/v1/books/latest | 
[**get_one_book**](BookControllerApi.md#get_one_book) | **GET** /api/v1/books/{bookId} | 
[**get_positions**](BookControllerApi.md#get_positions) | **GET** /api/v1/books/{bookId}/positions | 
[**get_progression**](BookControllerApi.md#get_progression) | **GET** /api/v1/books/{bookId}/progression | 
[**get_web_pub_manifest**](BookControllerApi.md#get_web_pub_manifest) | **GET** /api/v1/books/{bookId}/manifest | 
[**get_web_pub_manifest_divina**](BookControllerApi.md#get_web_pub_manifest_divina) | **GET** /api/v1/books/{bookId}/manifest/divina | 
[**get_web_pub_manifest_epub**](BookControllerApi.md#get_web_pub_manifest_epub) | **GET** /api/v1/books/{bookId}/manifest/epub | 
[**get_web_pub_manifest_pdf**](BookControllerApi.md#get_web_pub_manifest_pdf) | **GET** /api/v1/books/{bookId}/manifest/pdf | 
[**import_books**](BookControllerApi.md#import_books) | **POST** /api/v1/books/import | 
[**mark_progression**](BookControllerApi.md#mark_progression) | **PUT** /api/v1/books/{bookId}/progression | 
[**mark_read_progress**](BookControllerApi.md#mark_read_progress) | **PATCH** /api/v1/books/{bookId}/read-progress | 
[**mark_selected_book_thumbnail**](BookControllerApi.md#mark_selected_book_thumbnail) | **PUT** /api/v1/books/{bookId}/thumbnails/{thumbnailId}/selected | 
[**refresh_metadata2**](BookControllerApi.md#refresh_metadata2) | **POST** /api/v1/books/{bookId}/metadata/refresh | 
[**regenerate_thumbnails**](BookControllerApi.md#regenerate_thumbnails) | **PUT** /api/v1/books/thumbnails | 
[**update_batch_metadata**](BookControllerApi.md#update_batch_metadata) | **PATCH** /api/v1/books/metadata | 
[**update_metadata1**](BookControllerApi.md#update_metadata1) | **PATCH** /api/v1/books/{bookId}/metadata | 



## add_user_uploaded_book_thumbnail

> crate::models::ThumbnailBookDto add_user_uploaded_book_thumbnail(book_id, file, selected)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**selected** | Option<**bool**> |  |  |

### Return type

[**crate::models::ThumbnailBookDto**](ThumbnailBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analyze3

> analyze3(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_book

> delete_book(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_read_progress

> delete_read_progress(book_id)


Mark book as unread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_uploaded_book_thumbnail

> delete_user_uploaded_book_thumbnail(book_id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_books

> crate::models::PageBookDto get_all_books(search, library_id, media_status, read_status, released_after, tag, unpaged, page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**media_status** | Option<[**Vec<String>**](String.md)> |  |  |
**read_status** | Option<[**Vec<String>**](String.md)> |  |  |
**released_after** | Option<**String**> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PageBookDto**](PageBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_read_lists_by_book

> Vec<crate::models::ReadListDto> get_all_read_lists_by_book(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ReadListDto>**](ReadListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_file

> serde_json::Value get_book_file(book_id)


Download the book file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_file1

> serde_json::Value get_book_file1(book_id)


Download the book file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_page

> std::path::PathBuf get_book_page(book_id, page_number, convert, zero_based, accept)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**page_number** | **i32** |  | [required] |
**convert** | Option<**String**> | Convert the image to the provided format. |  |
**zero_based** | Option<**bool**> | If set to true, pages will start at index 0. If set to false, pages will start at index 1. |  |[default to false]
**accept** | Option<[**Vec<crate::models::MediaType>**](crate::models::MediaType.md)> | Some very limited server driven content negotiation is handled. If a book is a PDF book, and the Accept header contains 'application/pdf' as a more specific type than other 'image/' types, a raw PDF page will be returned. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_page_raw

> String get_book_page_raw(book_id, page_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**page_number** | **i32** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_page_thumbnail

> std::path::PathBuf get_book_page_thumbnail(book_id, page_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**page_number** | **i32** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_pages

> Vec<crate::models::PageDto> get_book_pages(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PageDto>**](PageDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_resource

> String get_book_resource(book_id, resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**resource** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_sibling_next1

> crate::models::BookDto get_book_sibling_next1(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::BookDto**](BookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_sibling_previous1

> crate::models::BookDto get_book_sibling_previous1(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::BookDto**](BookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_thumbnail

> std::path::PathBuf get_book_thumbnail(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_thumbnail_by_id

> std::path::PathBuf get_book_thumbnail_by_id(book_id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_thumbnails

> Vec<crate::models::ThumbnailBookDto> get_book_thumbnails(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ThumbnailBookDto>**](ThumbnailBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_books_on_deck

> crate::models::PageBookDto get_books_on_deck(library_id, page, size)


Return first unread book of series with at least one book read and no books in progress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | Option<[**Vec<String>**](String.md)> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageBookDto**](PageBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_duplicate_books

> crate::models::PageBookDto get_duplicate_books(unpaged, page, size, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sorting criteria in the format: property(,asc|desc). Default sort order is ascending. Multiple sort criteria are supported. |  |

### Return type

[**crate::models::PageBookDto**](PageBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_books

> crate::models::PageBookDto get_latest_books(unpaged, page, size)


Return newly added or updated books.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unpaged** | Option<**bool**> |  |  |
**page** | Option<**i32**> | Zero-based page index (0..N) |  |
**size** | Option<**i32**> | The size of the page to be returned |  |

### Return type

[**crate::models::PageBookDto**](PageBookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_book

> crate::models::BookDto get_one_book(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::BookDto**](BookDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_positions

> crate::models::R2Positions get_positions(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::R2Positions**](R2Positions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/vnd.readium.position-list+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_progression

> crate::models::R2Progression get_progression(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::R2Progression**](R2Progression.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/vnd.readium.progression+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_web_pub_manifest

> crate::models::WpPublicationDto get_web_pub_manifest(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::WpPublicationDto**](WPPublicationDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/webpub+json, application/divina+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_web_pub_manifest_divina

> crate::models::WpPublicationDto get_web_pub_manifest_divina(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::WpPublicationDto**](WPPublicationDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/divina+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_web_pub_manifest_epub

> crate::models::WpPublicationDto get_web_pub_manifest_epub(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::WpPublicationDto**](WPPublicationDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/webpub+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_web_pub_manifest_pdf

> crate::models::WpPublicationDto get_web_pub_manifest_pdf(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

[**crate::models::WpPublicationDto**](WPPublicationDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/webpub+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_books

> import_books(book_import_batch_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_import_batch_dto** | [**BookImportBatchDto**](BookImportBatchDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_progression

> mark_progression(book_id, r2_progression)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**r2_progression** | [**R2Progression**](R2Progression.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_read_progress

> mark_read_progress(book_id, read_progress_update_dto)


Mark book as read and/or change page progress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**read_progress_update_dto** | [**ReadProgressUpdateDto**](ReadProgressUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_selected_book_thumbnail

> mark_selected_book_thumbnail(book_id, thumbnail_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**thumbnail_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_metadata2

> refresh_metadata2(book_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_thumbnails

> regenerate_thumbnails(for_bigger_result_only)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**for_bigger_result_only** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_batch_metadata

> update_batch_metadata(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**::std::collections::HashMap<String, crate::models::BookMetadataUpdateDto>**](BookMetadataUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metadata1

> update_metadata1(book_id, book_metadata_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** |  | [required] |
**book_metadata_update_dto** | [**BookMetadataUpdateDto**](BookMetadataUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

