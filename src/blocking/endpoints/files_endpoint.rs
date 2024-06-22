use super::Error;
use crate::{
    blocking::ConnectorClient,
    types::{
        files::{
            requests::{
                CreateTokenForFileRequest, CreateTokenQrCodeForFileRequest, GetAllFilesRequest,
                GetOwnFilesRequest, GetPeerFilesRequest, LoadPeerFileRequest, UploadFileRequest,
            },
            ConnectorFile,
        },
        tokens::{ConnectorToken, FileToken},
    },
};
use reqwest::Method;

/// All endpoints related to file management, like upload or download of files.
impl<'a> ConnectorClient<'a> {
    pub fn get_files(
        &self,
        files_query: &GetAllFilesRequest<'_>,
    ) -> Result<Vec<ConnectorFile>, Error> {
        self.request(
            &format!(
                "api/v2/Files?{}",
                serde_urlencoded::to_string(files_query).unwrap()
            ),
            Method::GET,
            None,
        )
    }

    pub fn upload_file(&self, _upload_body: &UploadFileRequest<'_>) -> Result<ConnectorFile, Error> {
        unimplemented!()
        // self.upload_file_internal("api/v2/Files/Own", upload_body)
    }

    pub fn get_own_files(
        &self,
        files_query: &GetOwnFilesRequest<'_>,
    ) -> Result<Vec<ConnectorFile>, Error> {
        self.request(
            &format!(
                "api/v2/Files/Own?{}",
                serde_urlencoded::to_string(files_query).unwrap()
            ),
            Method::GET,
            None,
        )
    }

    pub fn load_peer_file(
        &self,
        file_reference: &LoadPeerFileRequest,
    ) -> Result<Vec<ConnectorFile>, Error> {
        self.request(
            &format!(
                "api/v2/Files/Own?{}",
                serde_urlencoded::to_string(file_reference).unwrap()
            ),
            Method::GET,
            None,
        )
    }

    pub fn get_peer_files(
        &self,
        peer_req: &GetPeerFilesRequest<'_>,
    ) -> Result<Vec<ConnectorFile>, Error> {
        self.request(
            &format!(
                "api/v2/Files/Own?{}",
                serde_urlencoded::to_string(peer_req).unwrap()
            ),
            Method::GET,
            None,
        )
    }

    pub fn get_file(&self, file_id: &str) -> Result<Vec<ConnectorFile>, Error> {
        self.request(&format!("api/v2/Files/{file_id}",), Method::GET, None)
    }

    pub fn get_file_data(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        self.download(&format!("api/v2/Files/{file_id}/Download",), Method::GET)
    }

    pub fn get_file_qr_code(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        self.download_qr(&format!("api/v2/Files/{file_id}"), Method::GET, None)
    }

    pub fn get_file_token(
        &self,
        file_id: &str,
        req: Option<CreateTokenForFileRequest<'_>>,
    ) -> Result<ConnectorToken<FileToken>, Error> {
        self.request(
            &format!("api/v2/Files/{file_id}/Token",),
            Method::POST,
            Some(serde_json::to_string(&req.unwrap_or_default()).unwrap()),
        )
    }

    pub fn get_file_token_qr_code(
        &self,
        file_id: &str,
        req: Option<CreateTokenQrCodeForFileRequest<'_>>,
    ) -> Result<Vec<u8>, Error> {
        self.download_qr(
            &format!("api/v2/Files/{file_id}/Token"),
            Method::POST,
            req.map(|r| serde_json::to_string(&r).unwrap()),
        )
    }
}
