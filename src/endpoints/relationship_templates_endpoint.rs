use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    types::{
        relationship_templates::{
            requests::{CreateOwnRelationshipTemplateRequest, GetRelationshipTemplatesRequest},
            requests::{CreateTokenForOwnRelationshipTemplateRequest, GetTemplatesRequest},
            ConnectorRelationshipTemplate,
        },
        tokens::{ConnectorToken, FileToken},
    },
    ConnectorClient,
};

impl<'a> ConnectorClient<'a> {
    pub async fn get_relationship_templates(
        &self,
        request_params: Option<GetRelationshipTemplatesRequest<'_>>,
    ) -> Result<Vec<ConnectorRelationshipTemplate<serde_json::Value>>, crate::connector_errors::Error>
    {
        self.request(
            &format!(
                "api/v2/RelationshipTemplates{}",
                request_params
                    .map(|i| format!("?{}", serde_urlencoded::to_string(i).unwrap()))
                    .unwrap_or_default()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn get_own_relationship_templates(
        &self,
        request_params: Option<GetTemplatesRequest<'_>>,
    ) -> Result<Vec<ConnectorRelationshipTemplate<serde_json::Value>>, crate::connector_errors::Error>
    {
        self.request(
            &format!(
                "api/v2/RelationshipTemplates{}",
                request_params
                    .map(|i| format!("?{}", serde_urlencoded::to_string(i).unwrap()))
                    .unwrap_or_default()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn get_peer_relationship_templates(
        &self,
        request_params: Option<GetTemplatesRequest<'_>>,
    ) -> Result<Vec<ConnectorRelationshipTemplate<serde_json::Value>>, crate::connector_errors::Error>
    {
        self.request(
            &format!(
                "api/v2/RelationshipTemplates{}",
                request_params
                    .map(|i| format!("?{}", serde_urlencoded::to_string(i).unwrap()))
                    .unwrap_or_default()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn get_relationship_template_by_id<T: DeserializeOwned>(
        &self,
        id: &str,
    ) -> Result<ConnectorRelationshipTemplate<T>, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/RelationshipTemplates/{id}"),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn create_own_relationship_template<T: Serialize + DeserializeOwned>(
        &self,
        template: &CreateOwnRelationshipTemplateRequest<'_, T>,
    ) -> Result<ConnectorRelationshipTemplate<T>, crate::connector_errors::Error> {
        self.request(
            "api/v2/RelationshipTemplates/Own",
            Method::POST,
            Some(serde_json::to_string(template).unwrap()),
        )
        .await
    }

    pub async fn get_qr_code_for_relationship_template<T: Serialize>(
        &self,
        id: &str,
    ) -> Result<Vec<u8>, crate::connector_errors::Error> {
        self.download_qr(
            &format!("api/v2/RelationshipTemplates/{id}"),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn create_token_for_own_relationship_template(
        &self,
        id: &str,
        req: Option<CreateTokenForOwnRelationshipTemplateRequest>,
    ) -> Result<ConnectorToken<FileToken>, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/RelationshipTemplates/Own/{id}"),
            Method::POST,
            req.map(|r| serde_json::to_string(&r).unwrap()),
        )
        .await
    }
}
