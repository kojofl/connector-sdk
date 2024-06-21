use serde::{Deserialize, Serialize};

use crate::types::files::ConnectorFile;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorMessage<M> {
    pub id: String,
    pub content: M,
    pub created_by: String,
    pub created_by_device: String,
    pub recipients: Vec<ConnectorMessageRecipient>,
    pub created_at: String,
    pub attachments: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorMessageRecipient {
    pub address: String,
    pub relationship_id: String,
    pub received_at: Option<String>,
    pub received_by_device: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorMessageWithAttachments<M> {
    pub id: String,
    pub content: M,
    pub created_by: String,
    pub created_by_device: String,
    pub recipients: Vec<ConnectorMessageRecipient>,
    pub created_at: String,
    pub attachments: Vec<ConnectorFile>,
}
