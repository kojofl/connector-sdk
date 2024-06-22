use reqwest::Method;

use super::Error;
use crate::{
    blocking::ConnectorClient,
    types::attributes::{
        requests::{
            CreateAttributeRequest, ExecuteIdentityAttributeQueryRequest, GetAttributesRequest,
            GetValidAttributesRequest,
        },
        ConnectorAttribute,
    },
};

/// All endpoints to manage the attributes of the connector.
impl<'a> ConnectorClient<'a> {
    pub fn create_attribute(
        &self,
        attribute: &CreateAttributeRequest,
    ) -> Result<ConnectorAttribute, Error> {
        self.request(
            "api/v2/Attributes",
            Method::POST,
            Some(serde_json::to_string(attribute).unwrap()),
        )
    }

    pub fn get_attributes(
        &self,
        query: &GetAttributesRequest<'a>,
    ) -> Result<Vec<ConnectorAttribute>, Error> {
        self.request(
            &format!(
                "api/v2/Attributes?{}",
                serde_urlencoded::to_string(query).unwrap_or_default()
            ),
            Method::GET,
            None,
        )
    }

    pub fn get_attribute_by_id(&self, id: &str) -> Result<ConnectorAttribute, Error> {
        self.request(&format!("api/v2/Attributes/{id}"), Method::GET, None)
    }

    pub fn get_valid_attributes(
        &self,
        query: &GetValidAttributesRequest<'a>,
    ) -> Result<Vec<ConnectorAttribute>, Error> {
        self.request(
            &format!(
                "api/v2/Attributes/Valid?{}",
                serde_urlencoded::to_string(query).unwrap_or_default()
            ),
            Method::GET,
            None,
        )
    }

    pub fn execute_identity_query(
        &self,
        body: &ExecuteIdentityAttributeQueryRequest<'a>,
    ) -> Result<Vec<ConnectorAttribute>, Error> {
        self.request(
            "api/v2/Attributes/ExecuteIdentityAttributeQuery",
            Method::POST,
            Some(serde_json::to_string(body).unwrap()),
        )
    }
}
