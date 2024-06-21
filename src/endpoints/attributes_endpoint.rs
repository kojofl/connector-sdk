use reqwest::Method;

use crate::{
    types::attributes::{
        requests::{
            CreateAttributeRequest, ExecuteIdentityAttributeQueryRequest, GetAttributesRequest,
            GetValidAttributesRequest,
        },
        ConnectorAttribute,
    },
    ConnectorClient,
};

/// All endpoints to manage the attributes of the connector.
impl<'a> ConnectorClient<'a> {
    pub async fn create_attribute(
        &self,
        attribute: &CreateAttributeRequest,
    ) -> Result<ConnectorAttribute, crate::connector_errors::Error> {
        self.request(
            "api/v2/Attributes",
            Method::POST,
            Some(serde_json::to_string(attribute).unwrap()),
        )
        .await
    }

    pub async fn get_attributes(
        &self,
        query: &GetAttributesRequest<'a>,
    ) -> Result<Vec<ConnectorAttribute>, crate::connector_errors::Error> {
        self.request(
            &format!(
                "api/v2/Attributes?{}",
                serde_urlencoded::to_string(query).unwrap_or_default()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn get_attribute_by_id(
        &self,
        id: &str,
    ) -> Result<ConnectorAttribute, crate::connector_errors::Error> {
        self.request(&format!("api/v2/Attributes/{id}"), Method::GET, None)
            .await
    }

    pub async fn get_valid_attributes(
        &self,
        query: &GetValidAttributesRequest<'a>,
    ) -> Result<Vec<ConnectorAttribute>, crate::connector_errors::Error> {
        self.request(
            &format!(
                "api/v2/Attributes/Valid?{}",
                serde_urlencoded::to_string(query).unwrap_or_default()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn execute_identity_query(
        &self,
        body: &ExecuteIdentityAttributeQueryRequest<'a>,
    ) -> Result<Vec<ConnectorAttribute>, crate::connector_errors::Error> {
        self.request(
            "api/v2/Attributes/ExecuteIdentityAttributeQuery",
            Method::POST,
            Some(serde_json::to_string(body).unwrap()),
        )
        .await
    }
}
