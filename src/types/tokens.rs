use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorToken<T> {
    pub id: String,
    pub created_by: String,
    pub created_by_device: String,
    pub content: T,
    pub created_at: String,
    pub expires_at: String,
    pub secret_key: String,
    pub truncated_reference: String,
    pub is_ephemeral: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenReference {
    id: String,
    secret_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenReferenceTruncated {
    reference: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOwnTokenRequest<'a, T> {
    expires_at: &'a str,
    content: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral: Option<bool>,
}

#[derive(Serialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOwnTokensRequest<'a> {
    #[serde(rename = "")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<Vec<&'a str>>,
    #[serde(rename = "")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by_device: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<Vec<&'a str>>,
}

#[derive(Serialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPeerTokensRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<Vec<&'a str>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadPeerTokenByTruncatedReferenceRequest<'a> {
    reference: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadPeerTokenByReferenceRequest<'a> {
    pub id: &'a str,
    pub secret_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
}

#[derive(Serialize, Clone, Debug)]
pub enum LoadPeerTokenRequest<'a> {
    LoadPeerTokenByReferenceRequest(LoadPeerTokenByReferenceRequest<'a>),
    LoadPeerTokenByTruncatedReferenceRequest(LoadPeerTokenByTruncatedReferenceRequest<'a>),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileToken {
    pub file_id: String,
    pub secret_key: SecretKey,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecretKey {
    pub alg: u32,
    pub key: String,
}
