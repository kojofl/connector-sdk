use connector_sdk;

#[cfg(test)]
mod template_test {
    use connector_sdk::types::{
        connector_request::{
            AttributeQuery, ConnectorRequestContent, IdentityAttributeQuery,
            ReadAttributeRequestItem, RequestContent, RequestItem,
        },
        relationship_templates::{
            requests::{CreateOwnRelationshipTemplateRequest, GetRelationshipTemplatesRequest},
            RelationshipTemplateContent, TemplateContent,
        },
    };

    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_get_relationship_template() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client
            .get_relationship_templates(Some(GetRelationshipTemplatesRequest {
                created_at: None,
                expires_at: None,
                created_by: None,
                created_by_device: None,
                max_number_of_allocations: None,
                is_own: Some(false)
            }))
            .await
            .is_ok());
    }

    #[actix_rt::test]
    async fn test_get_own_relationship_template() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client.get_own_relationship_templates(None).await.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_relationship_template_by_id() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client
            .get_relationship_template_by_id::<serde_json::Value>("RLTNX7iFeI2G94AqYBDk")
            .await
            .is_ok());

        let templates = client
            .get_relationship_templates(None)
            .await
            .expect("No error in request");

        if let Some(template) = templates.get(0) {
            println!("{}", template.id);
        }
    }

    #[actix_rt::test]
    async fn test_create_relationship_template() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        let template = CreateOwnRelationshipTemplateRequest {
            max_number_of_allocations: Some(1),
            expires_at: "2025",
            content: TemplateContent::relationship(RelationshipTemplateContent {
                title: None,
                metadata: None,
                on_new_relationship: ConnectorRequestContent {
                    id: None,
                    expires_at: None,
                    items: vec![RequestContent::Item(RequestItem::Read(
                        ReadAttributeRequestItem {
                            title: None,
                            description: None,
                            response_metadata: None,
                            must_be_accepted: true,
                            query: AttributeQuery::Identity(IdentityAttributeQuery {
                                valid_from: None,
                                valid_to: None,
                                tags: None,
                                value_type: "BirthName".into(),
                            }),
                        },
                    ))],
                    title: None,
                    description: None,
                    metadata: None,
                },
                on_existing_relationship: None,
            }),
        };

        assert!(client
            .create_own_relationship_template(&template)
            .await
            .is_ok());
    }
}
