use serde::{Deserialize, Serialize};

use super::connector_request::ConnectorRequestContent;

pub mod requests;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRelationshipTemplate<T> {
    pub id: String,
    pub is_own: bool,
    pub max_number_of_allocations: Option<u32>,
    pub created_by: String,
    pub created_by_device: String,
    pub created_at: String,
    pub content: Box<TemplateContent<T>>,
    pub expires_at: Option<String>,
    pub truncated_reference: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TemplateContent<T> {
    RelationshipTemplateContent(RelationshipTemplateContent),
    Any(T),
}

impl TemplateContent<()> {
    pub fn relationship(template: RelationshipTemplateContent) -> TemplateContent<()> {
        TemplateContent::RelationshipTemplateContent(template)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
pub struct RelationshipTemplateContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub on_new_relationship: ConnectorRequestContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_existing_relationship: Option<ConnectorRequestContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationshipTemplateReference {
    id: String,
    secret_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TruncatedRelationshipTemplateReference {
    /// starting with 'VE9L' for a truncated reference to a token containing a RelationshipTemplateReference or
    /// starting with 'UkxU' for a direct truncated RelationshipTemplateReference
    reference: String,
}
