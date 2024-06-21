use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    types::relationship::{
        requests::{
            CreateRelationshipRequest, DecideRelationshipChangeRequest, GetRelationshipsRequest,
        },
        responses::ConnectorRelationship,
    },
    ConnectorClient,
};

impl<'a> ConnectorClient<'a> {
    pub async fn create_relationship<T: Serialize + DeserializeOwned>(
        &self,
        req: CreateRelationshipRequest<'_, T>,
    ) -> Result<ConnectorRelationship<T, serde_json::Value>, crate::connector_errors::Error> {
        self.request(
            "api/v2/Relationships",
            Method::POST,
            Some(serde_json::to_string(&req).unwrap()),
        )
        .await
    }

    pub async fn get_relationship<T: DeserializeOwned>(
        &self,
        id: &str,
    ) -> Result<ConnectorRelationship<T, serde_json::Value>, crate::connector_errors::Error> {
        self.request(&format!("api/v2/Relationships/{id}",), Method::GET, None)
            .await
    }

    pub async fn get_relationships(
        &self,
        req: GetRelationshipsRequest<'_>,
    ) -> Result<
        Vec<ConnectorRelationship<serde_json::Value, serde_json::Value>>,
        crate::connector_errors::Error,
    > {
        self.request(
            &format!(
                "api/v2/Relationships?{}",
                serde_urlencoded::to_string(req).unwrap()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn accept_relationship_change<C: Serialize>(
        &self,
        rel_id: &str,
        change_id: &str,
        req: &DecideRelationshipChangeRequest<C>,
    ) -> Result<
        Vec<ConnectorRelationship<serde_json::Value, serde_json::Value>>,
        crate::connector_errors::Error,
    > {
        self.request(
            &format!("api/v2/Relationships/{rel_id}/Changes/{change_id}/Accept",),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }

    pub async fn reject_relationship_change<C: Serialize>(
        &self,
        rel_id: &str,
        change_id: &str,
        req: &DecideRelationshipChangeRequest<C>,
    ) -> Result<
        Vec<ConnectorRelationship<serde_json::Value, serde_json::Value>>,
        crate::connector_errors::Error,
    > {
        self.request(
            &format!("api/v2/Relationships/{rel_id}/Changes/{change_id}/Reject",),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }
}
