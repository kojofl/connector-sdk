use std::collections::HashMap;

use chrono::Utc;
use serde::Deserialize;

use super::account::SyncInfo;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorHealth {
    pub is_healthy: bool,
    pub services: HashMap<String, HealthStatus>,
}

#[derive(Deserialize, Clone, Debug)]
pub enum HealthStatus {
    #[serde(alias = "healthy")]
    Healthy,
    #[serde(alias = "unhealthy")]
    Unhealthy,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorVersionInfo {
    pub build: String,
    pub commit: String,
    pub version: String,
    pub date: chrono::DateTime<Utc>,
    pub runtime_version: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestCount {
    pub since: chrono::DateTime<Utc>,
    pub request_count: u32,
    pub request_count_by_status: HashMap<String, u32>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorSupport {
    pub version: ConnectorVersionInfo,
    pub health: ConnectorHealth,
    pub configuration: HashMap<String, StringOrObject>,
    pub identity_info: SyncInfo,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum StringOrObject {
    String(String),
    Object(HashMap<String, serde_json::Value>),
}
