use connector_sdk;

#[cfg(test)]
mod attribute_tests {
    use std::collections::HashMap;

    use connector_sdk::types::attributes::{
        requests::{
            AttributeRequestContent, CreateAttributeRequest, ExecuteIdentityAttributeQueryRequest,
            GetAttributesRequest, GetValidAttributesRequest, ValidAttributeRequestContent,
        },
        ConnectorAttributeValue, ConnectorContent, IdentityAttribute, IdentityAttributeQuery,
    };

    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_create_attribute() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let att = CreateAttributeRequest {
            content: ConnectorContent::IdentityAttribute(IdentityAttribute {
                owner: "id1CnW8wbrsxXakhFBBvwfiiocpULgDhTpZU".into(),
                valid_from: None,
                valid_to: None,
                value: ConnectorAttributeValue {
                    _type: "DisplayName".into(),
                    extra: HashMap::from([(
                        "value".into(),
                        serde_json::to_value("value").unwrap(),
                    )]),
                },
                tags: None,
            }),
        };
        assert!(client.create_attribute(&att).await.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_attributes() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let att = GetAttributesRequest {
            created_at: Some("2023-01-27T13:24:55.189Z"),
            content: Some(AttributeRequestContent {
                is_technical: Some(false),
                ..AttributeRequestContent::default()
            }),
            ..GetAttributesRequest::default()
        };
        assert!(client.get_attributes(&att).await.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_attribute_by_id() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client
            .get_attribute_by_id("ATTiRdeDNsKK5ZXnXY3G")
            .await
            .is_ok());
    }

    #[actix_rt::test]
    async fn test_get_valid_attributes() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let att = GetValidAttributesRequest {
            content: Some(ValidAttributeRequestContent {
                owner: Some("id1CnW8wbrsxXakhFBBvwfiiocpULgDhTpZU"),
                is_technical: Some(false),
                ..ValidAttributeRequestContent::default()
            }),
            ..GetValidAttributesRequest::default()
        };
        assert!(client.get_valid_attributes(&att).await.is_ok());
    }

    #[actix_rt::test]
    async fn test_identity_query() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let att = ExecuteIdentityAttributeQueryRequest {
            query: IdentityAttributeQuery {
                value_type: "DisplayName",
                ..Default::default()
            },
        };

        assert!(client.execute_identity_query(&att).await.is_ok());
    }
}
