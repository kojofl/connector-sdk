use serde::{Deserialize, Serialize};

use crate::types::ShareInfo;

use super::{
    Confidentiality, ConnectorContent, IdentityAttributeQuery, RelationshipAttributeQuery,
    ThirdPartyAttributeQuery,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttributeRequest {
    pub content: ConnectorContent,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteIdentityAttributeQueryRequest<'a> {
    pub query: IdentityAttributeQuery<'a>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRelationshipAttributeQueryRequest {
    query: RelationshipAttributeQuery,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteThirdPartyRelationshipAttributeQueryRequest {
    query: ThirdPartyAttributeQuery,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetValidAttributesRequest<'a> {
    #[serde(flatten)]
    pub content: Option<ValidAttributeRequestContent<'a>>,
    pub succeeds: Option<&'a str>,
    pub succeeded_by: Option<&'a str>,
    #[serde(flatten)]
    pub share_info: Option<ShareInfo>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidAttributeRequestContent<'a> {
    #[serde(rename = "@type")]
    pub _type: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub owner: Option<&'a str>,
    pub key: Option<&'a str>,
    pub is_technical: Option<bool>,
    pub confidentiality: Option<Confidentiality>,
    // TODO: check whether this does the correct thing
    // #[serde(flatten, rename = "@type")]
    // value_type: Option<&'a str>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAttributesRequest<'a> {
    pub created_at: Option<&'a str>,
    #[serde(flatten)]
    pub content: Option<AttributeRequestContent<'a>>,
    pub succeeds: Option<&'a str>,
    pub succeeded_by: Option<&'a str>,
    #[serde(flatten)]
    pub share_info: Option<ShareInfo>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributeRequestContent<'a> {
    #[serde(rename = "@type")]
    pub _type: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub owner: Option<&'a str>,
    pub valid_from: Option<&'a str>,
    pub valid_to: Option<&'a str>,
    pub key: Option<&'a str>,
    pub is_technical: Option<bool>,
    pub confidentiality: Option<Confidentiality>,
    // TODO: check whether this does the correct thing
    // #[serde(flatten, rename = "@type")]
    // value_type: Option<&'a str>,
}
