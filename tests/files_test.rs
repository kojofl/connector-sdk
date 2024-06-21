use connector_sdk;

#[cfg(test)]
mod message_tests {
    use super::connector_sdk::ConnectorClient;

    use connector_sdk::types::files::requests::{
        GetAllFilesRequest, GetOwnFilesRequest, UploadFileRequest,
    };

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_get_files() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_files_req = GetAllFilesRequest::default();
        let res = client.get_files(&get_files_req).await;
        assert!(res.is_ok());
    }

    #[actix_rt::test]
    async fn test_upload_file() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let upload_req = UploadFileRequest {
            title: "My toml",
            description: None,
            expires_at: "2025",
            file_path: "./Cargo.toml".into(),
        };
        let res = client.upload_file(&upload_req).await;
        assert!(res.is_ok());
    }

    #[actix_rt::test]
    async fn test_get_own_files() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_files_req = GetOwnFilesRequest::default();
        let res = client.get_own_files(&get_files_req).await;
        assert!(res.is_ok());

        for f in res.unwrap() {
            assert!(f.is_own)
        }
    }

    #[actix_rt::test]
    async fn test_get_file_data() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_files_req = GetOwnFilesRequest::default();
        let res = client.get_own_files(&get_files_req).await;
        assert!(res.is_ok());

        let files = res.unwrap();
        let file = files.last().expect("some files to exist");
        let data_res = client.get_file_data(&file.id).await;
        assert!(data_res.is_ok())
    }

    #[actix_rt::test]
    async fn test_get_file_qr() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_files_req = GetOwnFilesRequest::default();
        let res = client.get_own_files(&get_files_req).await;
        assert!(res.is_ok());
        let files = res.unwrap();
        let file = files.last().expect("some files to exist");
        let data_res = client.get_file_qr_code(&file.id).await;
        assert!(data_res.is_ok());
        let data = data_res.unwrap();
        assert!(data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]));
    }

    #[actix_rt::test]
    async fn test_get_file_token() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_files_req = GetOwnFilesRequest::default();
        let res = client.get_own_files(&get_files_req).await;
        assert!(res.is_ok());
        let files = res.unwrap();
        let file = files.last().expect("some files to exist");
        let data_res = client.get_file_token(&file.id, None).await;
        assert!(data_res.is_ok());
        let token = data_res.unwrap();
        assert_eq!(token.content.file_id, file.id);
    }

    #[actix_rt::test]
    async fn test_get_qr_for_file_token() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_files_req = GetOwnFilesRequest::default();
        let res = client.get_own_files(&get_files_req).await;
        assert!(res.is_ok());
        let files = res.unwrap();
        let file = files.last().expect("some files to exist");
        let data_res = client.get_file_token_qr_code(&file.id, None).await;
        println!("{:?}", data_res);
        assert!(data_res.is_ok());
        let data = data_res.unwrap();
        assert!(data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]));
    }
}
