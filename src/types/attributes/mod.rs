use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ShareInfo;

pub mod requests;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorAttribute {
    pub id: String,
    pub created_at: String,
    pub content: ConnectorContent,
    pub succeeds: Option<String>,
    pub succeeded_by: Option<String>,
    pub share_info: Option<ShareInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "@type")]
pub enum ConnectorContent {
    IdentityAttribute(IdentityAttribute),
    RelationshipAttribute(RelationshipAttribute),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdentityAttribute {
    pub owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    pub value: ConnectorAttributeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipAttribute {
    pub owner: String,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub value: ConnectorAttributeValue,
    pub key: String,
    pub is_technical: bool,
    pub confidentiality: Confidentiality,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Confidentiality {
    #[serde(alias = "public")]
    #[default]
    PUBLIC,
    #[serde(alias = "private")]
    PRIVATE,
    #[serde(alias = "protected")]
    PROTECTED,
}

impl Display for Confidentiality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Confidentiality::PUBLIC => f.write_str("PUBLIC"),
            Confidentiality::PRIVATE => f.write_str("PRIVATE"),
            Confidentiality::PROTECTED => f.write_str("PROTECTED"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorAttributeValue {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
pub struct IdentityAttributeQuery<'a> {
    pub value_type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<&'a str>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
pub struct RelationshipAttributeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_to: Option<String>,
    key: String,
    owner: String,
    attribute_creation_hints: RelationshipAttributeCreationHints,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipAttributeCreationHints {
    title: String,
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_hints: Option<ValueHints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_technical: Option<bool>,
    confidentiality: Confidentiality,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValueHints {
    #[serde(rename = "@type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    edit_help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<ValueHintsValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<DefaultValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValueHintsValue {
    #[serde(rename = "@type")]
    _type: String,
    key: DefaultValue,
    display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DefaultValue {
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
pub struct ThirdPartyAttributeQuery {
    key: String,
    owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_to: Option<String>,
}
