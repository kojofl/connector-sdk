use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DecideRelationshipChangeRequest<T> {
    pub content: T,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelationshipRequest<'a, T>
where
    T: Serialize,
{
    template_id: &'a str,
    content: T,
}

#[derive(Serialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRelationshipsRequest<'a> {
    template_id: Option<Vec<&'a str>>,
    peer: Option<Vec<&'a str>>,
    status: Option<Vec<&'a str>>,
}
