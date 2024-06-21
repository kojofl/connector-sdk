use chrono::Utc;
use serde::Deserialize;

use super::{messages::message::ConnectorMessage, relationship::responses::ConnectorRelationship};

/// The enmeshed address and the public key of the identity associated with the connector.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentityInfo {
    pub address: String,
    pub public_key: String,
}

/// Date of the last sync run
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncInfo {
    pub last_sync_run: Option<SyncDate>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncDate {
    pub completed_at: chrono::DateTime<Utc>,
}

/// The result of a connector sync contains all messages and relationships received by the
/// connector since the last sync. Since it is not plausible that all [`ConnectorMessage`] or
/// [`ConnectorRelationship`] have the same content we use [`serde_json::Value`] to represent them.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorSyncResult {
    pub messages: Vec<ConnectorMessage<serde_json::Value>>,
    pub relationships: Vec<ConnectorRelationship<serde_json::Value, serde_json::Value>>,
}
