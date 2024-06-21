use serde::{Deserialize, Serialize};

use super::super::relationship_templates::ConnectorRelationshipTemplate;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRelationship<T, C> {
    pub id: String,
    pub template: ConnectorRelationshipTemplate<T>,
    pub status: ConnectorRelationshipStatus,
    pub peer: String,
    pub peer_identity: ConnectorIdentity,
    pub changes: Vec<ConnectorRelationshipChange<C>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorIdentity {
    pub address: String,
    pub public_key: String,
    pub realm: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRelationshipChange<T> {
    pub id: String,
    pub _type: ConnectorRelationshipChangeType,
    pub status: ConnectorRelationshipChangeStatus,
    pub request: ConnectorRelationshipChangeRequest<T>,
    pub response: Option<ConnectorRelationshipChangeResponse<T>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorRelationshipChangeStatus {
    #[serde(alias = "Pending")]
    PENDING,
    #[serde(alias = "Rejected")]
    REJECTED,
    #[serde(alias = "Accepted")]
    ACCEPTED,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorRelationshipChangeType {
    #[serde(alias = "Creation")]
    CREATION,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorRelationshipStatus {
    #[serde(alias = "Pending")]
    PENDING,
    #[serde(alias = "Active")]
    ACTIVE,
    #[serde(alias = "Rejected")]
    REJECTED,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRelationshipChangeRequest<T> {
    pub created_by: String,
    pub created_by_device: String,
    pub created_at: String,
    pub content: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRelationshipChangeResponse<T> {
    pub created_by: String,
    pub created_by_device: String,
    pub created_at: String,
    pub content: Option<T>,
}
