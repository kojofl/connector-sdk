use serde::Serialize;

#[derive(Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_device: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Recipients>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_subject: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_body: Option<Vec<String>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Recipients {
    address: Option<Vec<String>>,
    relationship_id: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SendMessageRequest<'a, M>
where
    M: Serialize,
{
    pub recipients: Vec<&'a str>,
    pub content: M,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<&'a str>>,
}
