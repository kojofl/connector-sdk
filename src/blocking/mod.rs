#[doc(hidden)]
pub mod endpoints;

use crate::{
    connector_errors::{ConnectorError, Error},
    types::files::requests::UploadFileRequest,
    ResponseWraper,
};
use reqwest::{
    blocking::{multipart::Part, Client, ClientBuilder, RequestBuilder},
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

pub struct ConnectorClient<'a> {
    base_url: &'a str,
    client: Client,
}

impl<'a> ConnectorClient<'a> {
    /// To construct a connector client a base url the connector is listening to and the configured
    /// api key is needed.
    pub fn new(base_url: &'a str, api_key: &str) -> Self {
        let mut header = HeaderMap::new();
        header.insert(
            HeaderName::from_str("X-API-KEY").unwrap(),
            HeaderValue::from_str(api_key).unwrap(),
        );

        let client = ClientBuilder::new()
            .default_headers(header)
            .build()
            .unwrap();
        Self { base_url, client }
    }

    #[must_use]
    #[doc(hidden)]
    pub fn http_client(&self) -> Client {
        self.client.clone()
    }

    #[doc(hidden)]
    pub fn request<'de, M>(
        &self,
        resource: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<M, Error>
    where
        M: DeserializeOwned,
    {
        let mut req = self.build_request(resource, method);

        if let Some(body) = body {
            req = req.body(body);
        }

        let response = req.send()?;

        let all_bytes = response.bytes()?;

        match serde_json::from_slice::<ResponseWraper>(&all_bytes) {
            Ok(r) => match r {
                ResponseWraper::Result(r) => match serde_json::from_value::<M>(r) {
                    Ok(p) => Ok(p),
                    Err(e) => {
                        panic!("Error Parsing Connector Response: {e:#?}")
                    }
                },
                ResponseWraper::Error(r) => {
                    Err(serde_json::from_value::<ConnectorError>(r).unwrap().into())
                }
            },
            Err(e) => panic!("Error Parsing Connector Response: {e:?}"),
        }
    }

    #[doc(hidden)]
    pub fn request_plain<M>(&self, resource: &str, method: Method) -> Result<M, Error>
    where
        M: DeserializeOwned,
    {
        let req = self.build_request(resource, method);

        let response = req.send()?;

        let all_bytes = response.bytes()?;

        match serde_json::from_slice::<M>(&all_bytes) {
            Ok(r) => Ok(r),
            Err(_e) => {
                if let ResponseWraper::Error(e) =
                    serde_json::from_slice::<ResponseWraper>(&all_bytes).unwrap()
                {
                    Err(serde_json::from_value::<ConnectorError>(e).unwrap().into())
                } else {
                    panic!("Could not convert response to error type!")
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn download(&self, resource: &str, method: Method) -> Result<Vec<u8>, Error> {
        let req = self.build_download(resource, method);
        let response = req.send()?;
        let all_bytes = response.bytes()?;
        if let Ok(ResponseWraper::Error(e)) = serde_json::from_slice::<ResponseWraper>(&all_bytes) {
            Err(serde_json::from_value::<ConnectorError>(e).unwrap().into())
        } else {
            Ok(all_bytes.into())
        }
    }

    #[doc(hidden)]
    pub fn download_qr(
        &self,
        resource: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<Vec<u8>, Error> {
        let mut req = self.build_download_qr(resource, method);
        if let Some(body) = body {
            req = req.body(body);
        }
        let response = req.send()?;
        let all_bytes = response.bytes()?;
        if let Ok(ResponseWraper::Error(e)) = serde_json::from_slice::<ResponseWraper>(&all_bytes) {
            Err(serde_json::from_value::<ConnectorError>(e).unwrap().into())
        } else {
            Ok(all_bytes.into())
        }
    }

    #[doc(hidden)]
    pub fn upload_file_internal<'de, M>(
        &self,
        resource: &str,
        body: &UploadFileRequest<'_>,
    ) -> Result<M, Error>
    where
        M: DeserializeOwned,
    {
        let UploadFileRequest {
            title,
            description,
            expires_at,
            file_path,
        } = body;

        let file_name = file_path
            .file_name()
            .expect("A file to be uploaded")
            .to_string_lossy()
            .to_string();

        let file_data = std::fs::read(file_path).unwrap();

        let mut form = reqwest::blocking::multipart::Form::new()
            .text("title", (*title).to_owned())
            .text("expiresAt", (*expires_at).to_owned())
            .part("file", Part::bytes(file_data).file_name(file_name));
        if let Some(d) = description {
            form = form.text("description", (*d).to_owned());
        }

        let response = self.build_multipart(resource).multipart(form).send()?;

        let all_bytes = response.bytes()?;

        match serde_json::from_slice::<ResponseWraper>(&all_bytes) {
            Ok(r) => match r {
                ResponseWraper::Result(r) => match serde_json::from_value::<M>(r) {
                    Ok(p) => Ok(p),
                    Err(e) => panic!("Error Parsing Connector Response: {e:#?}"),
                },
                ResponseWraper::Error(r) => {
                    Err(serde_json::from_value::<ConnectorError>(r).unwrap().into())
                }
            },
            Err(e) => panic!("Error Parsing Connector Response: {e:?}"),
        }
    }

    #[doc(hidden)]
    fn build_request(&self, resource: &str, method: Method) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(method, url)
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
    }

    #[doc(hidden)]
    fn build_download(&self, resource: &str, method: Method) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(method, url)
            .header("accept", "application/octet-stream")
    }

    #[doc(hidden)]
    fn build_download_qr(&self, resource: &str, method: Method) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(method, url)
            .header("accept", "image/png")
    }

    #[doc(hidden)]
    fn build_multipart(&self, resource: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(Method::POST, url)
            .header("accept", "application/json")
    }
}
