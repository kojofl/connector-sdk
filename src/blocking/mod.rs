#[doc(hidden)]
pub mod endpoints;
pub mod errors;

use ureq::Agent;

use crate::{connector_errors::ConnectorError, ResponseWraper};
use errors::Error;
use reqwest::Method;
use serde::de::DeserializeOwned;

pub struct ConnectorClient<'a> {
    base_url: &'a str,
    api_key: &'a str,
    client: Agent,
}

impl<'a> ConnectorClient<'a> {
    /// To construct a connector client a base url the connector is listening to and the configured
    /// api key is needed.
    pub fn new(base_url: &'a str, api_key: &'a str) -> Self {
        let client = Agent::new();
        Self {
            base_url,
            api_key,
            client,
        }
    }

    #[must_use]
    #[doc(hidden)]
    pub fn http_client(&self) -> Agent {
        self.client.clone()
    }

    #[doc(hidden)]
    pub fn request<M>(
        &self,
        resource: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<M, Error>
    where
        M: DeserializeOwned,
    {
        let req = self.build_request(resource, method);

        let mut all_bytes = Vec::new();

        if let Some(body) = body {
            req.send_string(body.as_str())?
                .into_reader()
                .read_to_end(&mut all_bytes)
                .unwrap();
        } else {
            req.call()?
                .into_reader()
                .read_to_end(&mut all_bytes)
                .unwrap();
        }

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

        let response = req.call()?;

        let mut all_bytes = Vec::new();
        response.into_reader().read_to_end(&mut all_bytes).unwrap();

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
        let mut all_bytes = Vec::new();
        req.call()?
            .into_reader()
            .read_to_end(&mut all_bytes)
            .unwrap();
        if let Ok(ResponseWraper::Error(e)) = serde_json::from_slice::<ResponseWraper>(&all_bytes) {
            Err(serde_json::from_value::<ConnectorError>(e).unwrap().into())
        } else {
            Ok(all_bytes)
        }
    }

    #[doc(hidden)]
    pub fn download_qr(
        &self,
        resource: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<Vec<u8>, Error> {
        let req = self.build_download_qr(resource, method);
        let mut all_bytes = Vec::new();
        if let Some(body) = body {
            req.send_string(body.as_str())?
                .into_reader()
                .read_to_end(&mut all_bytes)
                .unwrap();
        } else {
            req.call()?
                .into_reader()
                .read_to_end(&mut all_bytes)
                .unwrap();
        }
        if let Ok(ResponseWraper::Error(e)) = serde_json::from_slice::<ResponseWraper>(&all_bytes) {
            Err(serde_json::from_value::<ConnectorError>(e).unwrap().into())
        } else {
            Ok(all_bytes)
        }
    }

    // TODO: find a way to implement this in a blocking manner
    // #[doc(hidden)]
    // pub fn upload_file_internal<'de, M>(
    //     &self,
    //     resource: &str,
    //     body: &UploadFileRequest<'_>,
    // ) -> Result<M, Error>
    // where
    //     M: DeserializeOwned,
    // {
    //     let UploadFileRequest {
    //         title,
    //         description,
    //         expires_at,
    //         file_path,
    //     } = body;

    //     let file_name = file_path
    //         .file_name()
    //         .expect("A file to be uploaded")
    //         .to_string_lossy()
    //         .to_string();

    //     let file_data = std::fs::read(file_path).unwrap();

    //     let mut form = reqwest::blocking::multipart::Form::new()
    //         .text("title", (*title).to_owned())
    //         .text("expiresAt", (*expires_at).to_owned())
    //         .part("file", Part::bytes(file_data).file_name(file_name));
    //     if let Some(d) = description {
    //         form = form.text("description", (*d).to_owned());
    //     }

    //     let response = self.build_multipart(resource, form).send()?;

    //     let all_bytes = response.bytes()?;

    //     match serde_json::from_slice::<ResponseWraper>(&all_bytes) {
    //         Ok(r) => match r {
    //             ResponseWraper::Result(r) => match serde_json::from_value::<M>(r) {
    //                 Ok(p) => Ok(p),
    //                 Err(e) => panic!("Error Parsing Connector Response: {e:#?}"),
    //             },
    //             ResponseWraper::Error(r) => {
    //                 Err(serde_json::from_value::<ConnectorError>(r).unwrap().into())
    //             }
    //         },
    //         Err(e) => panic!("Error Parsing Connector Response: {e:?}"),
    //     }
    // }

    #[doc(hidden)]
    fn build_request(&self, resource: &str, method: Method) -> ureq::Request {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(method.as_str(), url.as_str())
            .set("X-API-KEY", self.api_key)
            .set("Content-Type", "application/json")
            .set("accept", "application/json")
    }

    #[doc(hidden)]
    fn build_download(&self, resource: &str, method: Method) -> ureq::Request {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(method.as_str(), url.as_str())
            .set("X-API-KEY", self.api_key)
            .set("accept", "application/octet-stream")
    }

    #[doc(hidden)]
    fn build_download_qr(&self, resource: &str, method: Method) -> ureq::Request {
        let url = format!("{}/{}", self.base_url, resource);

        self.http_client()
            .request(method.as_str(), url.as_str())
            .set("accept", "image/png")
    }

    // TODO: find a way to implement this in a blocking manner
    // #[doc(hidden)]
    // fn build_multipart(&self, resource: &str, form: multipart::Form) -> ureq::Request {
    //     let url = format!("{}/{}", self.base_url, resource);

    //     reqwest::Client::default()
    //         .request(Method::POST, url)
    //         .header("accept", "application/json")
    //         .multipart(form)
    //         .build()
    //         .into()
    // }
}
