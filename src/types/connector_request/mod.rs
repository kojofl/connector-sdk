use serde::{Deserialize, Serialize};
pub mod get_requests;
pub mod requests;

use self::requests::{DecideRequestContent, DecideRequestItem};

use super::attributes::{RelationshipAttributeQuery, ThirdPartyAttributeQuery};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequest {
    pub id: String,
    pub is_own: bool,
    pub peer: String,
    pub created_at: String,
    pub status: ConnectorRequestStatus,
    pub content: ConnectorRequestContent,
    pub source: Option<ConnectorRequestSource>,
    pub response: Option<ConnectorRequestResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorRequestStatus {
    Draft,
    Open,
    DecisionRequired,
    ManualDecisionRequired,
    Decided,
    Completed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestSource {
    // "Message" | "RelationshipTemplate"
    _type: String,
    reference: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestResponse {
    pub created_at: String,
    pub content: ConnectorRequestResponseContent,
    pub source: Option<ConnectorRequestResponseSource>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestResponseSource {
    // "Message" | "RelationshipChange"
    pub _type: String,
    pub reference: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestResponseContent {
    #[serde(rename = "@type")]
    pub _type: String,
    pub result: ConnectorRequestResponseResult,
    pub request_id: String,
    pub items: Vec<RequestContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorRequestResponseResult {
    Accepted,
    Rejected,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestResponseItem {
    #[serde(rename = "@type")]
    pub _type: String,
    pub result: ConnectorRequestResponseItemResult,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectorRequestResponseItemResult {
    Accepted,
    Rejected,
    Failed,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestResponseItemGroup {
    #[serde(rename = "@type")]
    pub _type: String,
    pub items: Vec<ConnectorRequestResponseItem>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub items: Vec<RequestContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestContent {
    Item(RequestItem),
    Group(RequestItemGroup),
}

impl RequestContent {
    /// Convenience function to accept all request items in a given request.
    pub fn accept(&self) -> DecideRequestContent {
        match self {
            RequestContent::Item(_) => DecideRequestContent::Item(DecideRequestItem::Accept),
            RequestContent::Group(g) => {
                DecideRequestContent::Group(g.items.iter().map(|i| i.accept()).collect())
            }
        }
    }
    /// Convenience function to reject all request items in a given request.
    pub fn reject(&self) -> DecideRequestContent {
        match self {
            RequestContent::Item(_) => DecideRequestContent::Item(DecideRequestItem::reject()),
            RequestContent::Group(g) => {
                DecideRequestContent::Group(g.items.iter().map(|i| i.reject()).collect())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
pub struct RequestItemGroup {
    pub title: Option<String>,
    pub description: Option<String>,
    pub must_be_accepted: bool,
    pub metadata: Option<serde_json::Value>,
    pub items: Vec<RequestItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "@type")]
pub enum RequestItem {
    #[serde(rename = "CreateAttributeRequestItem")]
    Create(CreateAttributeRequestItem),
    #[serde(rename = "ShareAttributeRequestItem")]
    Share(ShareAttributeRequestItem),
    #[serde(rename = "ProposeAttributeRequestItem")]
    Propose(ProposeAttributeRequestItem),
    #[serde(rename = "ReadAttributeRequestItem")]
    Read(ReadAttributeRequestItem),
    #[serde(rename = "ConsentRequestItem")]
    Consent(ConsentRequestItem),
    #[serde(rename = "AuthenticationRequestItem")]
    Authentication(AuthenticationRequestItem),
}

impl RequestItem {
    pub fn accept(&self) -> DecideRequestItem {
        DecideRequestItem::Accept
    }
    pub fn reject(&self) -> DecideRequestItem {
        DecideRequestItem::reject()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttributeRequestItem {
    title: Option<String>,
    description: Option<String>,
    response_metadata: Option<serde_json::Value>,
    must_be_accepted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShareAttributeRequestItem {
    title: Option<String>,
    description: Option<String>,
    response_metadata: Option<serde_json::Value>,
    must_be_accepted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProposeAttributeRequestItem {
    title: Option<String>,
    description: Option<String>,
    response_metadata: Option<serde_json::Value>,
    must_be_accepted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReadAttributeRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<serde_json::Value>,
    pub must_be_accepted: bool,
    pub query: AttributeQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum AttributeQuery {
    Identity(IdentityAttributeQuery),
    Relationship(Box<RelationshipAttributeQuery>),
    ThirdParty(Box<ThirdPartyAttributeQuery>),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "@type")]
pub struct IdentityAttributeQuery {
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConsentRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<serde_json::Value>,
    pub must_be_accepted: bool,
    pub consent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_metadata: Option<serde_json::Value>,
    must_be_accepted: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorRequestValidationResult {
    pub is_success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub items: Vec<ConnectorRequestValidationResult>,
}
