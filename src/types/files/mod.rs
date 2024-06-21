use serde::{Deserialize, Serialize};

pub mod requests;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorFile {
    pub id: String,
    pub filename: String,
    pub filesize: f64,
    pub created_at: String,
    pub created_by: String,
    pub created_by_device: String,
    pub expires_at: String,
    pub mimetype: String,
    pub is_own: bool,
    pub title: String,
    pub description: Option<String>,
    pub secret_key: String,
    pub truncated_reference: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileReference {
    id: String,
    secret_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TruncatedFileReference {
    /**
     * starting with 'VE9L' for a truncated reference to a token containing a FileReference or
     * starting with 'RklM' for a direct truncated FileReference
     */
    reference: String,
}
