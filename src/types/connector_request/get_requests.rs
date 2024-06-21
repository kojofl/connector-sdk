use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response<'a>>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Content<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Items<'a>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Items<'a> {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<&'a str>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Source<'a> {
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ResponseContent<'a>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResponseContent<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ResponseItems<'a>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResponseItems<'a> {
    #[serde(rename = "@type")]
    pub type_: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Items<'a>>,
}
