<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-doubleclicksearch2` library allows access to all features of the *Google Doubleclicksearch* service.

This documentation was generated from *Doubleclicksearch* crate version *5.0.2+20230118*, where *20230118* is the exact revision of the *doubleclicksearch:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v5.0.2*.

Everything else about the *Doubleclicksearch* *v2* API can be found at the
[official documentation site](https://developers.google.com/search-ads).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/Doubleclicksearch) ... 

* [conversion](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::Conversion)
 * [*get*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ConversionGetCall), [*get by customer id*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ConversionGetByCustomerIdCall), [*insert*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ConversionInsertCall), [*update*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ConversionUpdateCall) and [*update availability*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ConversionUpdateAvailabilityCall)
* [reports](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::Report)
 * [*generate*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportGenerateCall), [*get*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportGetCall), [*get file*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportGetFileCall), [*get id mapping file*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportGetIdMappingFileCall) and [*request*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportRequestCall)
* [saved columns](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::SavedColumn)
 * [*list*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::SavedColumnListCall)


Download supported by ...

* [*get file reports*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportGetFileCall)
* [*get id mapping file reports*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/api::ReportGetIdMappingFileCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/Doubleclicksearch)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::CallBuilder)
* **[Resources](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.reports().generate(...).doit().await
let r = hub.reports().get(...).doit().await
let r = hub.reports().get_file(...).doit().await
let r = hub.reports().get_id_mapping_file(...).doit().await
let r = hub.reports().request(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-doubleclicksearch2 = "*"
serde = "^1.0"
serde_json = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_doubleclicksearch2 as doubleclicksearch2;
use doubleclicksearch2::{Result, Error};
use std::default::Default;
use doubleclicksearch2::{Doubleclicksearch, oauth2, hyper, hyper_rustls, chrono, FieldMask};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = Doubleclicksearch::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.reports().get_file("reportId", -33)
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Delegate) to the 
[Method Builder](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::RequestValue) and 
[decodable](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-doubleclicksearch2/5.0.2+20230118/google_doubleclicksearch2/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **doubleclicksearch2** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

