use serde::Serialize;

use super::{
    RelationshipTemplateReference, TemplateContent, TruncatedRelationshipTemplateReference,
};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOwnRelationshipTemplateRequest<'a, T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_allocations: Option<u32>,
    pub expires_at: &'a str,
    pub content: TemplateContent<T>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenForOwnRelationshipTemplateRequest {
    pub expires_at: Option<String>,
    pub ephemeral: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenQrCodeForOwnRelationshipTemplateRequest {
    pub expires_at: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTemplatesRequest<'a> {
    pub created_at: Option<&'a str>,
    pub expires_at: Option<&'a str>,
    pub created_by: Option<&'a str>,
    pub max_number_of_allocations: Option<u32>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRelationshipTemplatesRequest<'a> {
    pub created_at: Option<&'a str>,
    pub expires_at: Option<&'a str>,
    pub created_by: Option<&'a str>,
    pub created_by_device: Option<&'a str>,
    pub max_number_of_allocations: Option<u32>,
    pub is_own: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
pub enum LoadPeerRelationshipTemplateRequest {
    TruncatedRelationshipTemplateReference(TruncatedRelationshipTemplateReference),
    RelationshipTemplateReference(RelationshipTemplateReference),
}
