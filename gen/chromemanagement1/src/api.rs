use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See detailed information about apps installed on Chrome browsers and devices managed by your organization
    ChromeManagementAppdetailReadonly,

    /// See reports about devices and Chrome browsers managed within your organization
    ChromeManagementReportReadonly,

    /// See basic device and telemetry information collected from Chrome OS devices or users managed within your organization
    ChromeManagementTelemetryReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ChromeManagementAppdetailReadonly => "https://www.googleapis.com/auth/chrome.management.appdetails.readonly",
            Scope::ChromeManagementReportReadonly => "https://www.googleapis.com/auth/chrome.management.reports.readonly",
            Scope::ChromeManagementTelemetryReadonly => "https://www.googleapis.com/auth/chrome.management.telemetry.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ChromeManagementAppdetailReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all ChromeManagement related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// use chromemanagement1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_android_get("name")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct ChromeManagement<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for ChromeManagement<S> {}

impl<'a, S> ChromeManagement<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> ChromeManagement<S> {
        ChromeManagement {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.2".to_string(),
            _base_url: "https://chromemanagement.googleapis.com/".to_string(),
            _root_url: "https://chromemanagement.googleapis.com/".to_string(),
        }
    }

    pub fn customers(&'a self) -> CustomerMethods<'a, S> {
        CustomerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.2`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://chromemanagement.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://chromemanagement.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Android app information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AndroidAppInfo {
    /// Output only. Permissions requested by an Android app.
    
    pub permissions: Option<Vec<GoogleChromeManagementV1AndroidAppPermission>>,
}

impl client::Part for GoogleChromeManagementV1AndroidAppInfo {}


/// Permission requested by an Android app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AndroidAppPermission {
    /// Output only. The type of the permission.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1AndroidAppPermission {}


/// Resource representing app details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps android get customers](CustomerAppAndroidGetCall) (response)
/// * [apps chrome get customers](CustomerAppChromeGetCall) (response)
/// * [apps web get customers](CustomerAppWebGetCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AppDetails {
    /// Output only. Android app information.
    #[serde(rename="androidAppInfo")]
    
    pub android_app_info: Option<GoogleChromeManagementV1AndroidAppInfo>,
    /// Output only. Unique store identifier for the item. Examples: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension, "com.google.android.apps.docs" for the Google Drive Android app.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. Chrome Web Store app information.
    #[serde(rename="chromeAppInfo")]
    
    pub chrome_app_info: Option<GoogleChromeManagementV1ChromeAppInfo>,
    /// Output only. App's description.
    
    pub description: Option<String>,
    /// Output only. The uri for the detail page of the item.
    #[serde(rename="detailUri")]
    
    pub detail_uri: Option<String>,
    /// Output only. App's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. First published time.
    #[serde(rename="firstPublishTime")]
    
    pub first_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Home page or Website uri.
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<String>,
    /// Output only. A link to an image that can be used as an icon for the product.
    #[serde(rename="iconUri")]
    
    pub icon_uri: Option<String>,
    /// Output only. Indicates if the app has to be paid for OR has paid content.
    #[serde(rename="isPaidApp")]
    
    pub is_paid_app: Option<bool>,
    /// Output only. Latest published time.
    #[serde(rename="latestPublishTime")]
    
    pub latest_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version}
    
    pub name: Option<String>,
    /// Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="privacyPolicyUri")]
    
    pub privacy_policy_uri: Option<String>,
    /// Output only. The publisher of the item.
    
    pub publisher: Option<String>,
    /// Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app.
    #[serde(rename="reviewNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub review_number: Option<i64>,
    /// Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app.
    #[serde(rename="reviewRating")]
    
    pub review_rating: Option<f32>,
    /// Output only. App version. A new revision is committed whenever a new version of the app is published.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Output only. Information about a partial service error if applicable.
    #[serde(rename="serviceError")]
    
    pub service_error: Option<GoogleRpcStatus>,
    /// Output only. App type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleChromeManagementV1AppDetails {}


/// Status data for storage. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceAudioStatus](https://chromeenterprise.google/policies/#ReportDeviceAudioStatus) * Data Collection Frequency: 10 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1AudioStatusReport {
    /// Output only. Active input device's name.
    #[serde(rename="inputDevice")]
    
    pub input_device: Option<String>,
    /// Output only. Active input device's gain in [0, 100].
    #[serde(rename="inputGain")]
    
    pub input_gain: Option<i32>,
    /// Output only. Is active input device mute or not.
    #[serde(rename="inputMute")]
    
    pub input_mute: Option<bool>,
    /// Output only. Active output device's name.
    #[serde(rename="outputDevice")]
    
    pub output_device: Option<String>,
    /// Output only. Is active output device mute or not.
    #[serde(rename="outputMute")]
    
    pub output_mute: Option<bool>,
    /// Output only. Active output device's volume in [0, 100].
    #[serde(rename="outputVolume")]
    
    pub output_volume: Option<i32>,
    /// Output only. Timestamp of when the sample was collected on device.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1AudioStatusReport {}


/// Information about the battery. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDevicePowerStatus](https://chromeenterprise.google/policies/#ReportDevicePowerStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatteryInfo {
    /// Output only. Design capacity (mAmpere-hours).
    #[serde(rename="designCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub design_capacity: Option<i64>,
    /// Output only. Designed minimum output voltage (mV)
    #[serde(rename="designMinVoltage")]
    
    pub design_min_voltage: Option<i32>,
    /// Output only. The date the battery was manufactured.
    #[serde(rename="manufactureDate")]
    
    pub manufacture_date: Option<GoogleTypeDate>,
    /// Output only. Battery manufacturer.
    
    pub manufacturer: Option<String>,
    /// Output only. Battery serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Technology of the battery. Example: Li-ion
    
    pub technology: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BatteryInfo {}


/// Sampling data for battery. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDevicePowerStatus](https://chromeenterprise.google/policies/#ReportDevicePowerStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatterySampleReport {
    /// Output only. Battery charge percentage.
    #[serde(rename="chargeRate")]
    
    pub charge_rate: Option<i32>,
    /// Output only. Battery current (mA).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current: Option<i64>,
    /// Output only. The battery discharge rate measured in mW. Positive if the battery is being discharged, negative if it's being charged.
    #[serde(rename="dischargeRate")]
    
    pub discharge_rate: Option<i32>,
    /// Output only. Battery remaining capacity (mAmpere-hours).
    #[serde(rename="remainingCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub remaining_capacity: Option<i64>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Battery status read from sysfs. Example: Discharging
    
    pub status: Option<String>,
    /// Output only. Temperature in Celsius degrees.
    
    pub temperature: Option<i32>,
    /// Output only. Battery voltage (millivolt).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub voltage: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1BatterySampleReport {}


/// Status data for battery. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDevicePowerStatus](https://chromeenterprise.google/policies/#ReportDevicePowerStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BatteryStatusReport {
    /// Output only. Battery health.
    #[serde(rename="batteryHealth")]
    
    pub battery_health: Option<String>,
    /// Output only. Cycle count.
    #[serde(rename="cycleCount")]
    
    pub cycle_count: Option<i32>,
    /// Output only. Full charge capacity (mAmpere-hours).
    #[serde(rename="fullChargeCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub full_charge_capacity: Option<i64>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Sampling data for the battery sorted in a decreasing order of report_time.
    
    pub sample: Option<Vec<GoogleChromeManagementV1BatterySampleReport>>,
    /// Output only. Battery serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BatteryStatusReport {}


/// Boot performance report of a device. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceBootMode](https://chromeenterprise.google/policies/#ReportDeviceBootMode) * Data Collection Frequency: On every boot up event * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BootPerformanceReport {
    /// Total time to boot up.
    #[serde(rename="bootUpDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub boot_up_duration: Option<client::chrono::Duration>,
    /// The timestamp when power came on.
    #[serde(rename="bootUpTime")]
    
    pub boot_up_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Timestamp when the report was collected.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Total time since shutdown start to power off.
    #[serde(rename="shutdownDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub shutdown_duration: Option<client::chrono::Duration>,
    /// The shutdown reason.
    #[serde(rename="shutdownReason")]
    
    pub shutdown_reason: Option<String>,
    /// The timestamp when shutdown.
    #[serde(rename="shutdownTime")]
    
    pub shutdown_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1BootPerformanceReport {}


/// Describes a browser version and its install count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BrowserVersion {
    /// Output only. The release channel of the installed browser.
    
    pub channel: Option<String>,
    /// Output only. Count grouped by device_system and major version
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Output only. Version of the system-specified operating system.
    #[serde(rename="deviceOsVersion")]
    
    pub device_os_version: Option<String>,
    /// Output only. The device operating system.
    
    pub system: Option<String>,
    /// Output only. The full version of the installed browser.
    
    pub version: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BrowserVersion {}


/// Chrome Web Store app information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppInfo {
    /// Output only. Whether the app or extension is built and maintained by Google. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="googleOwned")]
    
    pub google_owned: Option<bool>,
    /// Output only. Whether the app or extension is in a published state in the Chrome Web Store.
    #[serde(rename="isCwsHosted")]
    
    pub is_cws_hosted: Option<bool>,
    /// Output only. Whether an app supports policy for extensions.
    #[serde(rename="isExtensionPolicySupported")]
    
    pub is_extension_policy_supported: Option<bool>,
    /// Output only. Whether the app is only for Kiosk mode on ChromeOS devices
    #[serde(rename="isKioskOnly")]
    
    pub is_kiosk_only: Option<bool>,
    /// Output only. Whether the app or extension is a theme.
    #[serde(rename="isTheme")]
    
    pub is_theme: Option<bool>,
    /// Output only. Whether this app is enabled for Kiosk mode on ChromeOS devices
    #[serde(rename="kioskEnabled")]
    
    pub kiosk_enabled: Option<bool>,
    /// Output only. The minimum number of users using this app.
    #[serde(rename="minUserCount")]
    
    pub min_user_count: Option<i32>,
    /// Output only. Every custom permission requested by the app. Version-specific field that will only be set when the requested app version is found.
    
    pub permissions: Option<Vec<GoogleChromeManagementV1ChromeAppPermission>>,
    /// Output only. Every permission giving access to domains or broad host patterns. ( e.g. www.google.com). This includes the matches from content scripts as well as hosts in the permissions node of the manifest. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="siteAccess")]
    
    pub site_access: Option<Vec<GoogleChromeManagementV1ChromeAppSiteAccess>>,
    /// Output only. The app developer has enabled support for their app. Version-specific field that will only be set when the requested app version is found.
    #[serde(rename="supportEnabled")]
    
    pub support_enabled: Option<bool>,
    /// Output only. Types of an item in the Chrome Web Store
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppInfo {}


/// Permission requested by a Chrome app or extension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppPermission {
    /// Output only. If available, whether this permissions grants the app/extension access to user data.
    #[serde(rename="accessUserData")]
    
    pub access_user_data: Option<bool>,
    /// Output only. If available, a URI to a page that has documentation for the current permission.
    #[serde(rename="documentationUri")]
    
    pub documentation_uri: Option<String>,
    /// Output only. The type of the permission.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppPermission {}


/// Details of an app installation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppRequest {
    /// Output only. Format: app_details=customers/{customer_id}/apps/chrome/{app_id}
    #[serde(rename="appDetails")]
    
    pub app_details: Option<String>,
    /// Output only. Unique store identifier for the app. Example: "gmbmikajjgmnabiglmofipeabaddhgne" for the Save to Google Drive Chrome extension.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. The uri for the detail page of the item.
    #[serde(rename="detailUri")]
    
    pub detail_uri: Option<String>,
    /// Output only. App's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A link to an image that can be used as an icon for the product.
    #[serde(rename="iconUri")]
    
    pub icon_uri: Option<String>,
    /// Output only. The timestamp of the most recently made request for this app.
    #[serde(rename="latestRequestTime")]
    
    pub latest_request_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Total count of requests for this app.
    #[serde(rename="requestCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_count: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppRequest {}


/// Represent one host permission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ChromeAppSiteAccess {
    /// Output only. This can contain very specific hosts, or patterns like "*.com" for instance.
    #[serde(rename="hostMatch")]
    
    pub host_match: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ChromeAppSiteAccess {}


/// Response containing summary of requested app installations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [apps count chrome app requests customers](CustomerAppCountChromeAppRequestCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeAppRequestsResponse {
    /// Token to specify the next page in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Count of requested apps matching request.
    #[serde(rename="requestedApps")]
    
    pub requested_apps: Option<Vec<GoogleChromeManagementV1ChromeAppRequest>>,
    /// Total number of matching app requests.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeAppRequestsResponse {}


/// Response containing a list of devices expiring in each month of a selected time frame. Counts are grouped by model and Auto Update Expiration date.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome devices reaching auto expiration date customers](CustomerReportCountChromeDevicesReachingAutoExpirationDateCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse {
    /// The list of reports sorted by auto update expiration date in ascending order.
    #[serde(rename="deviceAueCountReports")]
    
    pub device_aue_count_reports: Option<Vec<GoogleChromeManagementV1DeviceAueCountReport>>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse {}


/// Response containing counts for devices that need attention.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome devices that need attention customers](CustomerReportCountChromeDevicesThatNeedAttentionCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse {
    /// Number of ChromeOS devices have not synced policies in the past 28 days.
    #[serde(rename="noRecentPolicySyncCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub no_recent_policy_sync_count: Option<i64>,
    /// Number of ChromeOS devices that have not seen any user activity in the past 28 days.
    #[serde(rename="noRecentUserActivityCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub no_recent_user_activity_count: Option<i64>,
    /// Number of devices whose OS version is not compliant.
    #[serde(rename="osVersionNotCompliantCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub os_version_not_compliant_count: Option<i64>,
    /// Number of devices that are pending an OS update.
    #[serde(rename="pendingUpdate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pending_update: Option<i64>,
    /// Number of devices that are unable to apply a policy due to an OS version mismatch.
    #[serde(rename="unsupportedPolicyCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub unsupported_policy_count: Option<i64>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse {}


/// Response containing a list of devices with a specific type of hardware specification from the requested hardware type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome hardware fleet devices customers](CustomerReportCountChromeHardwareFleetDeviceCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse {
    /// The DeviceHardwareCountReport for device cpu type (for example Intel(R) Core(TM) i7-10610U CPU @ 1.80GHz).
    #[serde(rename="cpuReports")]
    
    pub cpu_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
    /// The DeviceHardwareCountReport for device memory amount in gigabytes (for example 16).
    #[serde(rename="memoryReports")]
    
    pub memory_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
    /// The DeviceHardwareCountReport for device model type (for example Acer C7 Chromebook).
    #[serde(rename="modelReports")]
    
    pub model_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
    /// The DeviceHardwareCountReport for device storage amount in gigabytes (for example 128).
    #[serde(rename="storageReports")]
    
    pub storage_reports: Option<Vec<GoogleChromeManagementV1DeviceHardwareCountReport>>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse {}


/// Response containing requested browser versions details and counts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome versions customers](CustomerReportCountChromeVersionCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
    /// List of all browser versions and their install counts.
    #[serde(rename="browserVersions")]
    
    pub browser_versions: Option<Vec<GoogleChromeManagementV1BrowserVersion>>,
    /// Token to specify the next page of the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number browser versions matching request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeVersionsResponse {}


/// Response containing details of queried installed apps.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count installed apps customers](CustomerReportCountInstalledAppCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
    /// List of installed apps matching request.
    #[serde(rename="installedApps")]
    
    pub installed_apps: Option<Vec<GoogleChromeManagementV1InstalledApp>>,
    /// Token to specify the next page of the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of installed apps matching request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountInstalledAppsResponse {}


/// CPU specifications for the device * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuInfo {
    /// Output only. Architecture type for the CPU. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
    
    pub architecture: Option<String>,
    /// Output only. Whether keylocker is configured.`TRUE` = Enabled; `FALSE` = disabled. Only reported if keylockerSupported = `TRUE`.
    #[serde(rename="keylockerConfigured")]
    
    pub keylocker_configured: Option<bool>,
    /// Output only. Whether keylocker is supported.
    #[serde(rename="keylockerSupported")]
    
    pub keylocker_supported: Option<bool>,
    /// Output only. The max CPU clock speed in kHz.
    #[serde(rename="maxClockSpeed")]
    
    pub max_clock_speed: Option<i32>,
    /// Output only. The CPU model name. Example: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz
    
    pub model: Option<String>,
}

impl client::Part for GoogleChromeManagementV1CpuInfo {}


/// Provides information about the status of the CPU. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Every 10 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuStatusReport {
    /// Output only. CPU temperature sample info per CPU core in Celsius
    #[serde(rename="cpuTemperatureInfo")]
    
    pub cpu_temperature_info: Option<Vec<GoogleChromeManagementV1CpuTemperatureInfo>>,
    /// Output only. Sample of CPU utilization (0-100 percent).
    #[serde(rename="cpuUtilizationPct")]
    
    pub cpu_utilization_pct: Option<i32>,
    /// Output only. The timestamp in milliseconds representing time at which this report was sampled.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_frequency: Option<client::chrono::Duration>,
}

impl client::Part for GoogleChromeManagementV1CpuStatusReport {}


/// CPU temperature of a device. Sampled per CPU core in Celsius. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceCpuInfo](https://chromeenterprise.google/policies/#ReportDeviceCpuInfo) * Data Collection Frequency: Every 10 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CpuTemperatureInfo {
    /// Output only. CPU label. Example: Core 0
    
    pub label: Option<String>,
    /// Output only. CPU temperature in Celsius.
    #[serde(rename="temperatureCelsius")]
    
    pub temperature_celsius: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1CpuTemperatureInfo {}


/// Describes a device reporting Chrome browser information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1Device {
    /// Output only. The ID of the device that reported this Chrome browser information.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. The name of the machine within its local network.
    
    pub machine: Option<String>,
}

impl client::Part for GoogleChromeManagementV1Device {}


/// Report for CountChromeDevicesPerAueDateResponse, contains the count of devices of a specific model and auto update expiration range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DeviceAueCountReport {
    /// Enum value of month corresponding to the auto update expiration date in UTC time zone. If the device is already expired, this field is empty.
    #[serde(rename="aueMonth")]
    
    pub aue_month: Option<String>,
    /// Int value of year corresponding to the Auto Update Expiration date in UTC time zone. If the device is already expired, this field is empty.
    #[serde(rename="aueYear")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub aue_year: Option<i64>,
    /// Count of devices of this model.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Boolean value for whether or not the device has already expired.
    
    pub expired: Option<bool>,
    /// Public model name of the devices.
    
    pub model: Option<String>,
}

impl client::Part for GoogleChromeManagementV1DeviceAueCountReport {}


/// Report for CountChromeDevicesPerHardwareSpecResponse, contains the count of devices with a unique hardware specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DeviceHardwareCountReport {
    /// Public name of the hardware specification.
    
    pub bucket: Option<String>,
    /// Count of devices with a unique hardware specification.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1DeviceHardwareCountReport {}


/// Status of the single storage device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DiskInfo {
    /// Output only. Number of bytes read since last boot.
    #[serde(rename="bytesReadThisSession")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_read_this_session: Option<i64>,
    /// Output only. Number of bytes written since last boot.
    #[serde(rename="bytesWrittenThisSession")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_written_this_session: Option<i64>,
    /// Output only. Time spent discarding since last boot. Discarding is writing to clear blocks which are no longer in use. Supported on kernels 4.18+.
    #[serde(rename="discardTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub discard_time_this_session: Option<client::chrono::Duration>,
    /// Output only. Disk health.
    
    pub health: Option<String>,
    /// Output only. Counts the time the disk and queue were busy, so unlike the fields above, parallel requests are not counted multiple times.
    #[serde(rename="ioTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub io_time_this_session: Option<client::chrono::Duration>,
    /// Output only. Disk manufacturer.
    
    pub manufacturer: Option<String>,
    /// Output only. Disk model.
    
    pub model: Option<String>,
    /// Output only. Time spent reading from disk since last boot.
    #[serde(rename="readTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub read_time_this_session: Option<client::chrono::Duration>,
    /// Output only. Disk serial number.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Disk size.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// Output only. Disk type: eMMC / NVMe / ATA / SCSI.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Output only. Disk volumes.
    #[serde(rename="volumeIds")]
    
    pub volume_ids: Option<Vec<String>>,
    /// Output only. Time spent writing to disk since last boot.
    #[serde(rename="writeTimeThisSession")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub write_time_this_session: Option<client::chrono::Duration>,
}

impl client::Part for GoogleChromeManagementV1DiskInfo {}


/// Information for a display.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1DisplayInfo {
    /// Output only. Represents the graphics card device id.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Output only. Indicates if display is internal or not.
    #[serde(rename="isInternal")]
    
    pub is_internal: Option<bool>,
    /// Output only. Refresh rate in Hz.
    #[serde(rename="refreshRate")]
    
    pub refresh_rate: Option<i32>,
    /// Output only. Resolution height in pixels.
    #[serde(rename="resolutionHeight")]
    
    pub resolution_height: Option<i32>,
    /// Output only. Resolution width in pixels.
    #[serde(rename="resolutionWidth")]
    
    pub resolution_width: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1DisplayInfo {}


/// Response containing a list of devices with queried app installed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports find installed app devices customers](CustomerReportFindInstalledAppDeviceCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
    /// A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field.
    
    pub devices: Option<Vec<GoogleChromeManagementV1Device>>,
    /// Token to specify the next page of the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of devices matching request.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1FindInstalledAppDevicesResponse {}


/// Information of a graphics adapter (GPU).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsAdapterInfo {
    /// Output only. Adapter name. Example: Mesa DRI Intel(R) UHD Graphics 620 (Kabylake GT2).
    
    pub adapter: Option<String>,
    /// Output only. Represents the graphics card device id.
    #[serde(rename="deviceId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub device_id: Option<i64>,
    /// Output only. Version of the GPU driver.
    #[serde(rename="driverVersion")]
    
    pub driver_version: Option<String>,
}

impl client::Part for GoogleChromeManagementV1GraphicsAdapterInfo {}


/// Information of the graphics subsystem. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceGraphicsStatus](https://chromeenterprise.google/policies/#ReportDeviceGraphicsStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsInfo {
    /// Output only. Information about the graphics adapter (GPU).
    #[serde(rename="adapterInfo")]
    
    pub adapter_info: Option<GoogleChromeManagementV1GraphicsAdapterInfo>,
}

impl client::Part for GoogleChromeManagementV1GraphicsInfo {}


/// Information of the graphics subsystem. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceGraphicsInfo](https://chromeenterprise.google/policies/#ReportDeviceGraphicsInfo) * Data Collection Frequency: 3 hours. * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1GraphicsStatusReport {
    /// Output only. Information about the displays for the device.
    
    pub displays: Option<Vec<GoogleChromeManagementV1DisplayInfo>>,
    /// Output only. Time at which the graphics data was reported.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1GraphicsStatusReport {}


/// Data that describes the result of the HTTPS latency diagnostics routine, with the HTTPS requests issued to Google websites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1HttpsLatencyRoutineData {
    /// Output only. HTTPS latency if routine succeeded or failed because of HIGH_LATENCY or VERY_HIGH_LATENCY.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub latency: Option<client::chrono::Duration>,
    /// Output only. HTTPS latency routine problem if a problem occurred.
    
    pub problem: Option<String>,
}

impl client::Part for GoogleChromeManagementV1HttpsLatencyRoutineData {}


/// Describes an installed app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1InstalledApp {
    /// Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote).
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// Output only. How the app was installed.
    #[serde(rename="appInstallType")]
    
    pub app_install_type: Option<String>,
    /// Output only. Source of the installed app.
    #[serde(rename="appSource")]
    
    pub app_source: Option<String>,
    /// Output only. Type of the app.
    #[serde(rename="appType")]
    
    pub app_type: Option<String>,
    /// Output only. Count of browser devices with this app installed.
    #[serde(rename="browserDeviceCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub browser_device_count: Option<i64>,
    /// Output only. Description of the installed app.
    
    pub description: Option<String>,
    /// Output only. Whether the app is disabled.
    
    pub disabled: Option<bool>,
    /// Output only. Name of the installed app.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Homepage uri of the installed app.
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<String>,
    /// Output only. Count of ChromeOS users with this app installed.
    #[serde(rename="osUserCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub os_user_count: Option<i64>,
    /// Output only. Permissions of the installed app.
    
    pub permissions: Option<Vec<String>>,
}

impl client::Part for GoogleChromeManagementV1InstalledApp {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [telemetry devices list customers](CustomerTelemetryDeviceListCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ListTelemetryDevicesResponse {
    /// Telemetry devices returned in the response.
    
    pub devices: Option<Vec<GoogleChromeManagementV1TelemetryDevice>>,
    /// Token to specify next page in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleChromeManagementV1ListTelemetryDevicesResponse {}


/// Response message for listing telemetry events for a customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [telemetry events list customers](CustomerTelemetryEventListCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ListTelemetryEventsResponse {
    /// Token to specify next page in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Telemetry events returned in the response.
    #[serde(rename="telemetryEvents")]
    
    pub telemetry_events: Option<Vec<GoogleChromeManagementV1TelemetryEvent>>,
}

impl client::ResponseResult for GoogleChromeManagementV1ListTelemetryEventsResponse {}


/// Memory information of a device. * This field has both telemetry and device information: - `totalRamBytes` - Device information - `availableRamBytes` - Telemetry information - `totalMemoryEncryption` - Device information * Data for this field is controlled via policy: [ReportDeviceMemoryInfo](https://chromeenterprise.google/policies/#ReportDeviceMemoryInfo) * Data Collection Frequency: - `totalRamBytes` - Only at upload - `availableRamBytes` - Every 10 minutes - `totalMemoryEncryption` - at device startup * Default Data Reporting Frequency: - `totalRamBytes` - 3 hours - `availableRamBytes` - 3 hours - `totalMemoryEncryption` - at device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: only for `totalMemoryEncryption` * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1MemoryInfo {
    /// Output only. Amount of available RAM in bytes.
    #[serde(rename="availableRamBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub available_ram_bytes: Option<i64>,
    /// Output only. Total memory encryption info for the device.
    #[serde(rename="totalMemoryEncryption")]
    
    pub total_memory_encryption: Option<GoogleChromeManagementV1TotalMemoryEncryptionInfo>,
    /// Output only. Total RAM in bytes.
    #[serde(rename="totalRamBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_ram_bytes: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1MemoryInfo {}


/// Contains samples of memory status reports. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceMemoryInfo](https://chromeenterprise.google/policies/#ReportDeviceMemoryInfo) * Data Collection Frequency: Only at upload, SystemRamFreeByes is collected every 10 minutes * Default Data Reporting Frequency: Every 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1MemoryStatusReport {
    /// Output only. Number of page faults during this collection
    #[serde(rename="pageFaults")]
    
    pub page_faults: Option<i32>,
    /// Output only. The timestamp in milliseconds representing time at which this report was sampled.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_frequency: Option<client::chrono::Duration>,
    /// Output only. Amount of free RAM in bytes (unreliable due to Garbage Collection).
    #[serde(rename="systemRamFreeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub system_ram_free_bytes: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1MemoryStatusReport {}


/// Details about the network device. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportNetworkDeviceConfiguration](https://chromeenterprise.google/policies/#ReportNetworkDeviceConfiguration) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkDevice {
    /// Output only. The integrated circuit card ID associated with the device's sim card.
    
    pub iccid: Option<String>,
    /// Output only. IMEI (if applicable) of the corresponding network device.
    
    pub imei: Option<String>,
    /// Output only. MAC address (if applicable) of the corresponding network device.
    #[serde(rename="macAddress")]
    
    pub mac_address: Option<String>,
    /// Output only. The mobile directory number associated with the device's sim card.
    
    pub mdn: Option<String>,
    /// Output only. MEID (if applicable) of the corresponding network device.
    
    pub meid: Option<String>,
    /// Output only. Network device type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleChromeManagementV1NetworkDevice {}


/// Network testing results to determine the health of the device's network connection, for example whether the HTTPS latency is high or normal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkDiagnosticsReport {
    /// Output only. HTTPS latency test data.
    #[serde(rename="httpsLatencyData")]
    
    pub https_latency_data: Option<GoogleChromeManagementV1HttpsLatencyRoutineData>,
    /// Output only. Timestamp of when the diagnostics were collected.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1NetworkDiagnosticsReport {}


/// Network device information. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportNetworkDeviceConfiguration](https://chromeenterprise.google/policies/#ReportNetworkDeviceConfiguration) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkInfo {
    /// Output only. List of network devices.
    #[serde(rename="networkDevices")]
    
    pub network_devices: Option<Vec<GoogleChromeManagementV1NetworkDevice>>,
}

impl client::Part for GoogleChromeManagementV1NetworkInfo {}


/// State of visible/configured networks. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportNetworkStatus](https://chromeenterprise.google/policies/#ReportNetworkStatus) * Data Collection Frequency: 60 minutes * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: Yes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1NetworkStatusReport {
    /// Output only. Current connection state of the network.
    #[serde(rename="connectionState")]
    
    pub connection_state: Option<String>,
    /// Output only. Network connection type.
    #[serde(rename="connectionType")]
    
    pub connection_type: Option<String>,
    /// Output only. Whether the wifi encryption key is turned off.
    #[serde(rename="encryptionOn")]
    
    pub encryption_on: Option<bool>,
    /// Output only. Gateway IP address.
    #[serde(rename="gatewayIpAddress")]
    
    pub gateway_ip_address: Option<String>,
    /// Output only. Network connection guid.
    
    pub guid: Option<String>,
    /// Output only. LAN IP address.
    #[serde(rename="lanIpAddress")]
    
    pub lan_ip_address: Option<String>,
    /// Output only. Receiving bit rate measured in Megabits per second.
    #[serde(rename="receivingBitRateMbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub receiving_bit_rate_mbps: Option<i64>,
    /// Output only. Time at which the network state was reported.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Frequency the report is sampled.
    #[serde(rename="sampleFrequency")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub sample_frequency: Option<client::chrono::Duration>,
    /// Output only. Signal strength for wireless networks measured in decibels.
    #[serde(rename="signalStrengthDbm")]
    
    pub signal_strength_dbm: Option<i32>,
    /// Output only. Transmission bit rate measured in Megabits per second.
    #[serde(rename="transmissionBitRateMbps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transmission_bit_rate_mbps: Option<i64>,
    /// Output only. Transmission power measured in decibels.
    #[serde(rename="transmissionPowerDbm")]
    
    pub transmission_power_dbm: Option<i32>,
    /// Output only. Wifi link quality. Value ranges from [0, 70]. 0 indicates no signal and 70 indicates a strong signal.
    #[serde(rename="wifiLinkQuality")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub wifi_link_quality: Option<i64>,
    /// Output only. Wifi power management enabled
    #[serde(rename="wifiPowerManagementEnabled")]
    
    pub wifi_power_management_enabled: Option<bool>,
}

impl client::Part for GoogleChromeManagementV1NetworkStatusReport {}


/// Contains information regarding the current OS update status. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceOsUpdateStatus](https://chromeenterprise.google/policies/#ReportDeviceOsUpdateStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1OsUpdateStatus {
    /// Output only. Timestamp of the last reboot.
    #[serde(rename="lastRebootTime")]
    
    pub last_reboot_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Timestamp of the last update check.
    #[serde(rename="lastUpdateCheckTime")]
    
    pub last_update_check_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Timestamp of the last successful update.
    #[serde(rename="lastUpdateTime")]
    
    pub last_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. New platform version of the os image being downloaded and applied. It is only set when update status is OS_IMAGE_DOWNLOAD_IN_PROGRESS or OS_UPDATE_NEED_REBOOT. Note this could be a dummy "0.0.0.0" for OS_UPDATE_NEED_REBOOT status for some edge cases, e.g. update engine is restarted without a reboot.
    #[serde(rename="newPlatformVersion")]
    
    pub new_platform_version: Option<String>,
    /// Output only. New requested platform version from the pending updated kiosk app.
    #[serde(rename="newRequestedPlatformVersion")]
    
    pub new_requested_platform_version: Option<String>,
    /// Output only. Current state of the os update.
    #[serde(rename="updateState")]
    
    pub update_state: Option<String>,
}

impl client::Part for GoogleChromeManagementV1OsUpdateStatus {}


/// Status data for storage. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceStorageStatus](https://chromeenterprise.google/policies/#ReportDeviceStorageStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageInfo {
    /// The available space for user data storage in the device in bytes.
    #[serde(rename="availableDiskBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub available_disk_bytes: Option<i64>,
    /// The total space for user data storage in the device in bytes.
    #[serde(rename="totalDiskBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_disk_bytes: Option<i64>,
    /// Information for disk volumes
    
    pub volume: Option<Vec<GoogleChromeManagementV1StorageInfoDiskVolume>>,
}

impl client::Part for GoogleChromeManagementV1StorageInfo {}


/// Information for disk volumes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageInfoDiskVolume {
    /// Free storage space in bytes.
    #[serde(rename="storageFreeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_free_bytes: Option<i64>,
    /// Total storage space in bytes.
    #[serde(rename="storageTotalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_total_bytes: Option<i64>,
    /// Disk volume id.
    #[serde(rename="volumeId")]
    
    pub volume_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1StorageInfoDiskVolume {}


/// Status data for storage. * This field is telemetry information and this will change over time as the device is utilized. * Data for this field is controlled via policy: [ReportDeviceStorageStatus](https://chromeenterprise.google/policies/#ReportDeviceStorageStatus) * Data Collection Frequency: Only at Upload * Default Data Reporting Frequency: 3 hours - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: No * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1StorageStatusReport {
    /// Output only. Reports on disk.
    
    pub disk: Option<Vec<GoogleChromeManagementV1DiskInfo>>,
    /// Output only. Timestamp of when the sample was collected on device
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleChromeManagementV1StorageStatusReport {}


/// `TelemetryAudioSevereUnderrunEvent` is triggered when a audio devices run out of buffer data for more than 5 seconds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent { _never_set: Option<bool> }

impl client::Part for GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent {}


/// Telemetry data collected from a managed device.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [telemetry devices get customers](CustomerTelemetryDeviceGetCall) (response)
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryDevice {
    /// Output only. Audio reports collected periodically sorted in a decreasing order of report_time.
    #[serde(rename="audioStatusReport")]
    
    pub audio_status_report: Option<Vec<GoogleChromeManagementV1AudioStatusReport>>,
    /// Output only. Information on battery specs for the device.
    #[serde(rename="batteryInfo")]
    
    pub battery_info: Option<Vec<GoogleChromeManagementV1BatteryInfo>>,
    /// Output only. Battery reports collected periodically.
    #[serde(rename="batteryStatusReport")]
    
    pub battery_status_report: Option<Vec<GoogleChromeManagementV1BatteryStatusReport>>,
    /// Output only. Boot performance reports of the device.
    #[serde(rename="bootPerformanceReport")]
    
    pub boot_performance_report: Option<Vec<GoogleChromeManagementV1BootPerformanceReport>>,
    /// Output only. Information regarding CPU specs for the device.
    #[serde(rename="cpuInfo")]
    
    pub cpu_info: Option<Vec<GoogleChromeManagementV1CpuInfo>>,
    /// Output only. CPU status reports collected periodically sorted in a decreasing order of report_time.
    #[serde(rename="cpuStatusReport")]
    
    pub cpu_status_report: Option<Vec<GoogleChromeManagementV1CpuStatusReport>>,
    /// Output only. Google Workspace Customer whose enterprise enrolled the device.
    
    pub customer: Option<String>,
    /// Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the ChromeOS Devices tab
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. Contains information regarding Graphic peripherals for the device.
    #[serde(rename="graphicsInfo")]
    
    pub graphics_info: Option<GoogleChromeManagementV1GraphicsInfo>,
    /// Output only. Graphics reports collected periodically.
    #[serde(rename="graphicsStatusReport")]
    
    pub graphics_status_report: Option<Vec<GoogleChromeManagementV1GraphicsStatusReport>>,
    /// Output only. Information regarding memory specs for the device.
    #[serde(rename="memoryInfo")]
    
    pub memory_info: Option<GoogleChromeManagementV1MemoryInfo>,
    /// Output only. Memory status reports collected periodically sorted decreasing by report_time.
    #[serde(rename="memoryStatusReport")]
    
    pub memory_status_report: Option<Vec<GoogleChromeManagementV1MemoryStatusReport>>,
    /// Output only. Resource name of the device.
    
    pub name: Option<String>,
    /// Output only. Network diagnostics collected periodically.
    #[serde(rename="networkDiagnosticsReport")]
    
    pub network_diagnostics_report: Option<Vec<GoogleChromeManagementV1NetworkDiagnosticsReport>>,
    /// Output only. Network devices information.
    #[serde(rename="networkInfo")]
    
    pub network_info: Option<GoogleChromeManagementV1NetworkInfo>,
    /// Output only. Network specs collected periodically.
    #[serde(rename="networkStatusReport")]
    
    pub network_status_report: Option<Vec<GoogleChromeManagementV1NetworkStatusReport>>,
    /// Output only. Organization unit ID of the device.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
    /// Output only. Contains relevant information regarding ChromeOS update status.
    #[serde(rename="osUpdateStatus")]
    
    pub os_update_status: Option<Vec<GoogleChromeManagementV1OsUpdateStatus>>,
    /// Output only. Device serial number. This value is the same as the Admin Console's Serial Number in the ChromeOS Devices tab.
    #[serde(rename="serialNumber")]
    
    pub serial_number: Option<String>,
    /// Output only. Information of storage specs for the device.
    #[serde(rename="storageInfo")]
    
    pub storage_info: Option<GoogleChromeManagementV1StorageInfo>,
    /// Output only. Storage reports collected periodically.
    #[serde(rename="storageStatusReport")]
    
    pub storage_status_report: Option<Vec<GoogleChromeManagementV1StorageStatusReport>>,
    /// Output only. Information on Thunderbolt bus.
    #[serde(rename="thunderboltInfo")]
    
    pub thunderbolt_info: Option<Vec<GoogleChromeManagementV1ThunderboltInfo>>,
}

impl client::ResponseResult for GoogleChromeManagementV1TelemetryDevice {}


/// Information about a device associated with telemetry data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryDeviceInfo {
    /// Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the ChromeOS Devices tab.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Output only. Organization unit ID of the device.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1TelemetryDeviceInfo {}


/// Telemetry data reported by a managed device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryEvent {
    /// Output only. Payload for audio severe underrun event. Present only when the `event_type` field is `AUDIO_SEVERE_UNDERRUN`.
    #[serde(rename="audioSevereUnderrunEvent")]
    
    pub audio_severe_underrun_event: Option<GoogleChromeManagementV1TelemetryAudioSevereUnderrunEvent>,
    /// Output only. Information about the device associated with the event.
    
    pub device: Option<GoogleChromeManagementV1TelemetryDeviceInfo>,
    /// The event type of the current event.
    #[serde(rename="eventType")]
    
    pub event_type: Option<String>,
    /// Output only. Payload for HTTPS latency change event. Present only when `event_type` is `NETWORK_HTTPS_LATENCY_CHANGE`.
    #[serde(rename="httpsLatencyChangeEvent")]
    
    pub https_latency_change_event: Option<GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent>,
    /// Output only. Resource name of the event.
    
    pub name: Option<String>,
    /// Timestamp that represents when the event was reported.
    #[serde(rename="reportTime")]
    
    pub report_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Payload for usb peripherals event. Present only when the `event_type` field is either `USB_ADDED` or `USB_REMOVED`.
    #[serde(rename="usbPeripheralsEvent")]
    
    pub usb_peripherals_event: Option<GoogleChromeManagementV1TelemetryUsbPeripheralsEvent>,
    /// Output only. Information about the user associated with the event.
    
    pub user: Option<GoogleChromeManagementV1TelemetryUserInfo>,
}

impl client::Part for GoogleChromeManagementV1TelemetryEvent {}


/// Https latency routine is run periodically and `TelemetryHttpsLatencyChangeEvent` is triggered if a latency problem was detected or if the device has recovered from a latency problem..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent {
    /// HTTPS latency routine data that triggered the event.
    #[serde(rename="httpsLatencyRoutineData")]
    
    pub https_latency_routine_data: Option<GoogleChromeManagementV1HttpsLatencyRoutineData>,
    /// Current HTTPS latency state.
    #[serde(rename="httpsLatencyState")]
    
    pub https_latency_state: Option<String>,
}

impl client::Part for GoogleChromeManagementV1TelemetryHttpsLatencyChangeEvent {}


/// `TelemetryUsbPeripheralsEvent` is triggered USB devices are either added or removed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryUsbPeripheralsEvent {
    /// List of usb devices that were either added or removed.
    #[serde(rename="usbPeripheralReport")]
    
    pub usb_peripheral_report: Option<Vec<GoogleChromeManagementV1UsbPeripheralReport>>,
}

impl client::Part for GoogleChromeManagementV1TelemetryUsbPeripheralsEvent {}


/// Information about a user associated with telemetry data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TelemetryUserInfo {
    /// Output only. User's email.
    
    pub email: Option<String>,
    /// Output only. Organization unit ID of the user.
    #[serde(rename="orgUnitId")]
    
    pub org_unit_id: Option<String>,
}

impl client::Part for GoogleChromeManagementV1TelemetryUserInfo {}


/// Thunderbolt bus info. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceSecurityStatus](https://chromeenterprise.google/policies/#ReportDeviceSecurityStatus) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: No * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1ThunderboltInfo {
    /// Security level of the Thunderbolt bus.
    #[serde(rename="securityLevel")]
    
    pub security_level: Option<String>,
}

impl client::Part for GoogleChromeManagementV1ThunderboltInfo {}


/// Memory encryption information of a device. * This field provides device information, which is static and will not change over time. * Data for this field is controlled via policy: [ReportDeviceMemoryInfo](https://chromeenterprise.google/policies/#ReportDeviceMemoryInfo) * Data Collection Frequency: At device startup * Default Data Reporting Frequency: At device startup - Policy Controlled: Yes * Cache: If the device is offline, the collected data is stored locally, and will be reported when the device is next online: Yes * Reported for affiliated users only: N/A
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1TotalMemoryEncryptionInfo {
    /// Memory encryption algorithm.
    #[serde(rename="encryptionAlgorithm")]
    
    pub encryption_algorithm: Option<String>,
    /// The state of memory encryption on the device.
    #[serde(rename="encryptionState")]
    
    pub encryption_state: Option<String>,
    /// The length of the encryption keys.
    #[serde(rename="keyLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub key_length: Option<i64>,
    /// The maximum number of keys that can be used for encryption.
    #[serde(rename="maxKeys")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_keys: Option<i64>,
}

impl client::Part for GoogleChromeManagementV1TotalMemoryEncryptionInfo {}


/// USB connected peripheral report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1UsbPeripheralReport {
    /// Output only. Categories the device belongs to https://www.usb.org/defined-class-codes
    
    pub categories: Option<Vec<String>>,
    /// Output only. Class ID https://www.usb.org/defined-class-codes
    #[serde(rename="classId")]
    
    pub class_id: Option<i32>,
    /// Output only. Firmware version
    #[serde(rename="firmwareVersion")]
    
    pub firmware_version: Option<String>,
    /// Output only. Device name, model name, or product name
    
    pub name: Option<String>,
    /// Output only. Product ID
    
    pub pid: Option<i32>,
    /// Output only. Subclass ID https://www.usb.org/defined-class-codes
    #[serde(rename="subclassId")]
    
    pub subclass_id: Option<i32>,
    /// Output only. Vendor name
    
    pub vendor: Option<String>,
    /// Output only. Vendor ID
    
    pub vid: Option<i32>,
}

impl client::Part for GoogleChromeManagementV1UsbPeripheralReport {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`ChromeManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apps_android_get(...)`, `apps_chrome_get(...)`, `apps_count_chrome_app_requests(...)`, `apps_web_get(...)`, `reports_count_chrome_devices_reaching_auto_expiration_date(...)`, `reports_count_chrome_devices_that_need_attention(...)`, `reports_count_chrome_hardware_fleet_devices(...)`, `reports_count_chrome_versions(...)`, `reports_count_installed_apps(...)`, `reports_find_installed_app_devices(...)`, `telemetry_devices_get(...)`, `telemetry_devices_list(...)` and `telemetry_events_list(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_android_get(&self, name: &str) -> CustomerAppAndroidGetCall<'a, S> {
        CustomerAppAndroidGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_chrome_get(&self, name: &str) -> CustomerAppChromeGetCall<'a, S> {
        CustomerAppChromeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_web_get(&self, name: &str) -> CustomerAppWebGetCall<'a, S> {
        CustomerAppWebGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate summary of app installation requests.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn apps_count_chrome_app_requests(&self, customer: &str) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        CustomerAppCountChromeAppRequestCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of the number of devices expiring in each month of the selected time frame. Devices are grouped by auto update expiration date and model. Further information can be found [here](https://support.google.com/chrome/a/answer/10564947).
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The customer ID or "my_customer" prefixed with "customers/".
    pub fn reports_count_chrome_devices_reaching_auto_expiration_date(&self, customer: &str) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        CustomerReportCountChromeDevicesReachingAutoExpirationDateCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _org_unit_id: Default::default(),
            _min_aue_date: Default::default(),
            _max_aue_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Counts of ChromeOS devices that have not synced policies or have lacked user activity in the past 28 days, are out of date, or are not complaint. Further information can be found here https://support.google.com/chrome/a/answer/10564947
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The customer ID or "my_customer" prefixed with "customers/".
    pub fn reports_count_chrome_devices_that_need_attention(&self, customer: &str) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        CustomerReportCountChromeDevicesThatNeedAttentionCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _read_mask: Default::default(),
            _org_unit_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Counts of devices with a specific hardware specification from the requested hardware type (for example model name, processor type). Further information can be found here https://support.google.com/chrome/a/answer/10564947
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The customer ID or "my_customer".
    pub fn reports_count_chrome_hardware_fleet_devices(&self, customer: &str) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        CustomerReportCountChromeHardwareFleetDeviceCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _read_mask: Default::default(),
            _org_unit_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of installed Chrome versions.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_chrome_versions(&self, customer: &str) -> CustomerReportCountChromeVersionCall<'a, S> {
        CustomerReportCountChromeVersionCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of app installations.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_installed_apps(&self, customer: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        CustomerReportCountInstalledAppCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of devices that have a specified app installed.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_find_installed_app_devices(&self, customer: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        CustomerReportFindInstalledAppDeviceCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _app_type: Default::default(),
            _app_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get telemetry device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `TelemetryDevice` to return.
    pub fn telemetry_devices_get(&self, name: &str) -> CustomerTelemetryDeviceGetCall<'a, S> {
        CustomerTelemetryDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all telemetry devices.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn telemetry_devices_list(&self, parent: &str) -> CustomerTelemetryDeviceListCall<'a, S> {
        CustomerTelemetryDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List telemetry events.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn telemetry_events_list(&self, parent: &str) -> CustomerTelemetryEventListCall<'a, S> {
        CustomerTelemetryEventListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get a specific app for a customer by its resource name.
///
/// A builder for the *apps.android.get* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_android_get("name")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppAndroidGetCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerAppAndroidGetCall<'a, S> {}

impl<'a, S> CustomerAppAndroidGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1AppDetails)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.android.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerAppAndroidGetCall<'a, S> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppAndroidGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppAndroidGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementAppdetailReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerAppAndroidGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerAppAndroidGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerAppAndroidGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get a specific app for a customer by its resource name.
///
/// A builder for the *apps.chrome.get* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_chrome_get("name")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppChromeGetCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerAppChromeGetCall<'a, S> {}

impl<'a, S> CustomerAppChromeGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1AppDetails)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.chrome.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerAppChromeGetCall<'a, S> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppChromeGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppChromeGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementAppdetailReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerAppChromeGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerAppChromeGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerAppChromeGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get a specific app for a customer by its resource name.
///
/// A builder for the *apps.web.get* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_web_get("name")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppWebGetCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerAppWebGetCall<'a, S> {}

impl<'a, S> CustomerAppWebGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1AppDetails)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.web.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerAppWebGetCall<'a, S> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppWebGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppWebGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementAppdetailReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerAppWebGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerAppWebGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerAppWebGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Generate summary of app installation requests.
///
/// A builder for the *apps.countChromeAppRequests* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().apps_count_chrome_app_requests("customer")
///              .page_token("sed")
///              .page_size(-2)
///              .org_unit_id("takimata")
///              .order_by("amet.")
///              .doit().await;
/// # }
/// ```
pub struct CustomerAppCountChromeAppRequestCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerAppCountChromeAppRequestCall<'a, S> {}

impl<'a, S> CustomerAppCountChromeAppRequestCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeAppRequestsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.apps.countChromeAppRequests",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/apps:countChromeAppRequests";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementAppdetailReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 50, anything above will be coerced to 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported fields: * request_count * latest_request_time
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerAppCountChromeAppRequestCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementAppdetailReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerAppCountChromeAppRequestCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerAppCountChromeAppRequestCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Generate report of the number of devices expiring in each month of the selected time frame. Devices are grouped by auto update expiration date and model. Further information can be found [here](https://support.google.com/chrome/a/answer/10564947).
///
/// A builder for the *reports.countChromeDevicesReachingAutoExpirationDate* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_devices_reaching_auto_expiration_date("customer")
///              .org_unit_id("ipsum")
///              .min_aue_date("gubergren")
///              .max_aue_date("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _org_unit_id: Option<String>,
    _min_aue_date: Option<String>,
    _max_aue_date: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {}

impl<'a, S> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeDevicesReachingAutoExpirationDateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countChromeDevicesReachingAutoExpirationDate",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "orgUnitId", "minAueDate", "maxAueDate"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }
        if let Some(value) = self._min_aue_date.as_ref() {
            params.push("minAueDate", value);
        }
        if let Some(value) = self._max_aue_date.as_ref() {
            params.push("maxAueDate", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countChromeDevicesReachingAutoExpirationDate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The customer ID or "my_customer" prefixed with "customers/".
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Optional. The organizational unit ID, if omitted, will return data for all organizational units.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum expiration date in format yyyy-mm-dd in UTC timezone. If included returns all devices that have already expired and devices with auto expiration date equal to or later than the minimum date.
    ///
    /// Sets the *min aue date* query property to the given value.
    pub fn min_aue_date(mut self, new_value: &str) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        self._min_aue_date = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum expiration date in format yyyy-mm-dd in UTC timezone. If included returns all devices that have already expired and devices with auto expiration date equal to or earlier than the maximum date.
    ///
    /// Sets the *max aue date* query property to the given value.
    pub fn max_aue_date(mut self, new_value: &str) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        self._max_aue_date = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementReportReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Counts of ChromeOS devices that have not synced policies or have lacked user activity in the past 28 days, are out of date, or are not complaint. Further information can be found here https://support.google.com/chrome/a/answer/10564947
///
/// A builder for the *reports.countChromeDevicesThatNeedAttention* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_devices_that_need_attention("customer")
///              .read_mask(&Default::default())
///              .org_unit_id("eos")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _read_mask: Option<client::FieldMask>,
    _org_unit_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {}

impl<'a, S> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeDevicesThatNeedAttentionResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countChromeDevicesThatNeedAttention",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "readMask", "orgUnitId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countChromeDevicesThatNeedAttention";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The customer ID or "my_customer" prefixed with "customers/".
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Required. Mask of the fields that should be populated in the returned report.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        self._read_mask = Some(new_value);
        self
    }
    /// Optional. The ID of the organizational unit. If omitted, all data will be returned.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementReportReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Counts of devices with a specific hardware specification from the requested hardware type (for example model name, processor type). Further information can be found here https://support.google.com/chrome/a/answer/10564947
///
/// A builder for the *reports.countChromeHardwareFleetDevices* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_hardware_fleet_devices("customer")
///              .read_mask(&Default::default())
///              .org_unit_id("ea")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountChromeHardwareFleetDeviceCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _read_mask: Option<client::FieldMask>,
    _org_unit_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {}

impl<'a, S> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeHardwareFleetDevicesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countChromeHardwareFleetDevices",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "readMask", "orgUnitId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countChromeHardwareFleetDevices";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The customer ID or "my_customer".
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Required. Mask of the fields that should be populated in the returned report.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        self._read_mask = Some(new_value);
        self
    }
    /// Optional. The ID of the organizational unit. If omitted, all data will be returned.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementReportReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Generate report of installed Chrome versions.
///
/// A builder for the *reports.countChromeVersions* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_versions("customer")
///              .page_token("invidunt")
///              .page_size(-47)
///              .org_unit_id("duo")
///              .filter("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountChromeVersionCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerReportCountChromeVersionCall<'a, S> {}

impl<'a, S> CustomerReportCountChromeVersionCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeVersionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countChromeVersions",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countChromeVersions";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountChromeVersionCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementReportReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerReportCountChromeVersionCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerReportCountChromeVersionCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerReportCountChromeVersionCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Generate report of app installations.
///
/// A builder for the *reports.countInstalledApps* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_installed_apps("customer")
///              .page_token("ut")
///              .page_size(-12)
///              .org_unit_id("rebum.")
///              .order_by("est")
///              .filter("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountInstalledAppCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerReportCountInstalledAppCall<'a, S> {}

impl<'a, S> CustomerReportCountInstalledAppCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountInstalledAppsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countInstalledApps",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countInstalledApps";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountInstalledAppCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementReportReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerReportCountInstalledAppCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerReportCountInstalledAppCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerReportCountInstalledAppCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Generate report of devices that have a specified app installed.
///
/// A builder for the *reports.findInstalledAppDevices* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_find_installed_app_devices("customer")
///              .page_token("est")
///              .page_size(-62)
///              .org_unit_id("ea")
///              .order_by("dolor")
///              .filter("Lorem")
///              .app_type("eos")
///              .app_id("labore")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportFindInstalledAppDeviceCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _filter: Option<String>,
    _app_type: Option<String>,
    _app_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerReportFindInstalledAppDeviceCall<'a, S> {}

impl<'a, S> CustomerReportFindInstalledAppDeviceCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1FindInstalledAppDevicesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.findInstalledAppDevices",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy", "filter", "appType", "appId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("customer", self._customer);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._org_unit_id.as_ref() {
            params.push("orgUnitId", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._app_type.as_ref() {
            params.push("appType", value);
        }
        if let Some(value) = self._app_id.as_ref() {
            params.push("appId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:findInstalledAppDevices";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["customer"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the page of the request to be returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported order by fields: * machine * device_id
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Type of the app.
    ///
    /// Sets the *app type* query property to the given value.
    pub fn app_type(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._app_type = Some(new_value.to_string());
        self
    }
    /// Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote).
    ///
    /// Sets the *app id* query property to the given value.
    pub fn app_id(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._app_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportFindInstalledAppDeviceCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementReportReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerReportFindInstalledAppDeviceCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerReportFindInstalledAppDeviceCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get telemetry device.
///
/// A builder for the *telemetry.devices.get* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().telemetry_devices_get("name")
///              .read_mask(&Default::default())
///              .doit().await;
/// # }
/// ```
pub struct CustomerTelemetryDeviceGetCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _name: String,
    _read_mask: Option<client::FieldMask>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerTelemetryDeviceGetCall<'a, S> {}

impl<'a, S> CustomerTelemetryDeviceGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1TelemetryDevice)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.telemetry.devices.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "name", "readMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementTelemetryReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Name of the `TelemetryDevice` to return.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> CustomerTelemetryDeviceGetCall<'a, S> {
        self._name = new_value.to_string();
        self
    }
    /// Required. Read mask to specify which fields to return.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> CustomerTelemetryDeviceGetCall<'a, S> {
        self._read_mask = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerTelemetryDeviceGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerTelemetryDeviceGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementTelemetryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerTelemetryDeviceGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerTelemetryDeviceGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerTelemetryDeviceGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List all telemetry devices.
///
/// A builder for the *telemetry.devices.list* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().telemetry_devices_list("parent")
///              .read_mask(&Default::default())
///              .page_token("sed")
///              .page_size(-61)
///              .filter("Stet")
///              .doit().await;
/// # }
/// ```
pub struct CustomerTelemetryDeviceListCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _parent: String,
    _read_mask: Option<client::FieldMask>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerTelemetryDeviceListCall<'a, S> {}

impl<'a, S> CustomerTelemetryDeviceListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1ListTelemetryDevicesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.telemetry.devices.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "parent", "readMask", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+parent}/telemetry/devices";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementTelemetryReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._parent = new_value.to_string();
        self
    }
    /// Required. Read mask to specify which fields to return.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._read_mask = Some(new_value);
        self
    }
    /// Token to specify next page in the list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Default value is 100. Maximum value is 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. Only include resources that match the filter. Supported filter fields: - org_unit_id - serial_number - device_id 
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerTelemetryDeviceListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementTelemetryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerTelemetryDeviceListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerTelemetryDeviceListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerTelemetryDeviceListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List telemetry events.
///
/// A builder for the *telemetry.events.list* method supported by a *customer* resource.
/// It is not used directly, but through a [`CustomerMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().telemetry_events_list("parent")
///              .read_mask(&Default::default())
///              .page_token("et")
///              .page_size(-43)
///              .filter("et")
///              .doit().await;
/// # }
/// ```
pub struct CustomerTelemetryEventListCall<'a, S>
    where S: 'a {

    hub: &'a ChromeManagement<S>,
    _parent: String,
    _read_mask: Option<client::FieldMask>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for CustomerTelemetryEventListCall<'a, S> {}

impl<'a, S> CustomerTelemetryEventListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1ListTelemetryEventsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.telemetry.events.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "parent", "readMask", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+parent}/telemetry/events";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::ChromeManagementTelemetryReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> CustomerTelemetryEventListCall<'a, S> {
        self._parent = new_value.to_string();
        self
    }
    /// Required. Read mask to specify which fields to return.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> CustomerTelemetryEventListCall<'a, S> {
        self._read_mask = Some(new_value);
        self
    }
    /// Optional. Token to specify next page in the list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerTelemetryEventListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum number of results to return. Default value is 100. Maximum value is 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerTelemetryEventListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. Only include resources that match the filter. Supported filter fields: * device_id * user_id * device_org_unit_id * user_org_unit_id * timestamp * event_type
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerTelemetryEventListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerTelemetryEventListCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> CustomerTelemetryEventListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ChromeManagementTelemetryReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CustomerTelemetryEventListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CustomerTelemetryEventListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CustomerTelemetryEventListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


