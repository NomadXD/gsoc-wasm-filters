use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Period {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Eternity,
}

impl Period {
    pub fn as_secs(&self) -> u64 {
        match *self {
            Period::Minute => 60,
            Period::Hour => 3600,
            Period::Day => 86400,
            Period::Week => 604800,
            Period::Month => 2592000,
            Period::Year => 31536000,
            Period::Eternity => u64::MAX,
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PeriodWindow {
    pub start: Duration,
    pub end: Duration,
    pub window: Period,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct UsageReport {
    pub period_window: PeriodWindow,
    pub left_hits: u64,
    // Required to renew window untill new state is fetched from 3scale.
    pub max_value: u64,
}

#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppId(String);
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppKey(String);
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserKey(String);

impl AsRef<str> for AppId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for AppKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for UserKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for UserKey {
    fn from(a: &str) -> Self {
        Self(a.to_string())
    }
}

impl From<&str> for AppId {
    fn from(a: &str) -> Self {
        Self(a.to_string())
    }
}

impl From<&str> for AppKey {
    fn from(a: &str) -> Self {
        Self(a.to_string())
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceToken(String);
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceId(String);

impl AsRef<str> for ServiceToken {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ServiceId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for ServiceToken {
    fn from(a: &str) -> Self {
        Self(a.to_string())
    }
}

impl From<&str> for ServiceId {
    fn from(a: &str) -> Self {
        Self(a.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct CacheKey(ServiceId, AppIdentifier);

impl<'a> CacheKey {
    pub fn as_string(&self) -> String {
        format!("{}_{}", self.0 .0, self.1.as_string())
    }

    pub fn default() -> CacheKey {
        CacheKey(
            ServiceId(String::new()),
            AppIdentifier::UserKey(UserKey(String::new())),
        )
    }

    pub fn service_id(&'a self) -> &'a ServiceId {
        &self.0
    }

    pub fn app_id(&'a self) -> &'a AppIdentifier {
        &self.1
    }

    pub fn from(a: &ServiceId, b: &AppIdentifier) -> CacheKey {
        CacheKey {
            0: a.clone(),
            1: b.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AppIdentifier {
    AppId(AppId, Option<AppKey>),
    UserKey(UserKey),
}

impl From<AppId> for AppIdentifier {
    fn from(a: AppId) -> Self {
        AppIdentifier::AppId(a, None)
    }
}

impl From<(AppId, AppKey)> for AppIdentifier {
    fn from(a: (AppId, AppKey)) -> Self {
        AppIdentifier::AppId(a.0, Some(a.1))
    }
}

impl From<UserKey> for AppIdentifier {
    fn from(u: UserKey) -> Self {
        AppIdentifier::UserKey(u)
    }
}

impl AppIdentifier {
    // This cannot return a reference because app_id:app_key needs to be generated
    pub fn as_string(&self) -> String {
        match self {
            AppIdentifier::AppId(AppId(id), key) => {
                let mut res: String = id.clone();
                if let Some(AppKey(app_key)) = key {
                    res.push(':');
                    res.push_str(app_key.as_str());
                }
                res
            }
            AppIdentifier::UserKey(UserKey(user_key)) => user_key.clone(),
        }
    }

    pub fn appid_from_str(s: &str) -> AppIdentifier {
        let v: Vec<&str> = s.split(':').collect();
        if v.len() == 1 {
            return AppIdentifier::AppId(AppId(v[0].to_owned()), Some(AppKey(v[1].to_owned())));
        }
        AppIdentifier::AppId(AppId(v[0].to_owned()), None)
    }
}

// Threescale's Application representation for cache
#[derive(Serialize, Deserialize)]
pub struct Application {
    pub app_id: AppIdentifier,
    pub service_id: ServiceId,
    pub local_state: HashMap<String, UsageReport>,
    pub metric_hierarchy: HashMap<String, Vec<String>>,
}

// Request data recieved from previous filters
#[derive(Serialize, Deserialize, Clone)]
pub struct ThreescaleData {
    // TODO: App_key, user_key is also possible as an input
    pub app_id: AppIdentifier,
    pub service_id: ServiceId,
    pub service_token: ServiceToken,
    pub metrics: RefCell<HashMap<String, u64>>,
}

impl Default for ThreescaleData {
    fn default() -> Self {
        ThreescaleData {
            app_id: AppIdentifier::UserKey(UserKey("".to_owned())),
            service_id: ServiceId("".to_owned()),
            service_token: ServiceToken("".to_owned()),
            metrics: RefCell::new(HashMap::new()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub update_cache_from_singleton: bool,
    pub data: ThreescaleData,
}

impl Message {
    pub fn new(update_flag: bool, request_data: &ThreescaleData) -> Message {
        Message {
            update_cache_from_singleton: update_flag,
            data: request_data.clone(),
        }
    }
}
