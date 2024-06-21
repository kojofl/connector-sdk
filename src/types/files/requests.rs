use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{FileReference, TruncatedFileReference};

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenForFileRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenQrCodeForFileRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllFilesRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_device: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_own: Option<bool>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOwnFilesRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_device: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<&'a str>>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPeerFilesRequest<'a> {
    created_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filesize: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mimetype: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<&'a str>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileRequest<'a> {
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    pub expires_at: &'a str,
    pub file_path: PathBuf,
}

#[derive(Serialize, Clone, Debug)]
pub enum LoadPeerFileRequest {
    TruncatedFileReference(TruncatedFileReference),
    FileReference(FileReference),
}
