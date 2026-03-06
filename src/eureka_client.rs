use crate::errors::EurekaClientError;
use crate::request::RegisterRequest;
use crate::response::{ApplicationResponse, ApplicationsResponse};
use hyper::header::{
    ACCEPT, ACCEPT_CHARSET, ACCEPT_ENCODING, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT,
};
use hyper::{body, Body, Client, Method, Request, StatusCode, Uri};
use serde_json;

/// A client for accessing Eureka
pub struct EurekaClient {
    client: Client<hyper::client::HttpConnector>,
    client_name: String,
    eureka_cluster_url: String,
}

//
// A simple port of the example found at: https://github.com/Netflix/eureka/wiki/Example-Custom-ReadOnly-client
// Eureka REST API: https://github.com/Netflix/eureka/wiki/Eureka-REST-operations
impl EurekaClient {
    /// Creates a new instance of EurekaClient
    ///
    /// # Arguments
    ///
    /// * `client_name` - The name of this client
    /// * `eureka_cluster_url` - The base url to the eureka cluster
    pub fn new(client_name: &str, eureka_cluster_url: &str) -> EurekaClient {
        debug!(
            "Creating new Eureka Client client_name:{:?}, eureka_client:{:?}",
            client_name, eureka_cluster_url
        );
        EurekaClient {
            client: Client::new(),
            client_name: client_name.to_owned(),
            eureka_cluster_url: eureka_cluster_url.to_owned(),
        }
    }

    pub async fn register(
        &self,
        application_id: &str,
        register_request: &RegisterRequest,
    ) -> Result<(), EurekaClientError> {
        debug!(
            "register: application_id={:?}, register_request:{:?}",
            application_id, register_request
        );
        let path = format!("/v2/apps/{}", application_id);
        let uri = self.build_uri(&path)?;

        let json = serde_json::to_string(register_request).map_err(EurekaClientError::from)?;

        let mut req = Request::builder().method(Method::POST).uri(uri);

        req = self.set_headers(req);

        let req = req
            .header(CONTENT_LENGTH, json.len())
            .body(Body::from(json))
            .map_err(|e| EurekaClientError::GenericError(e.to_string()))?;

        let res = self.client.request(req).await?;

        debug!("register: server response {:?}", res);

        match res.status() {
            StatusCode::BAD_REQUEST => Err(EurekaClientError::BadRequest),
            StatusCode::INTERNAL_SERVER_ERROR => Err(EurekaClientError::InternalServerError),
            _ => Ok(()),
        }
    }

    pub async fn get_application(
        &self,
        application_id: &str,
    ) -> Result<ApplicationResponse, EurekaClientError> {
        let path = format!("/v2/apps/{}", application_id);
        let uri = self.build_uri(&path)?;

        let mut req = Request::builder().method(Method::GET).uri(uri);

        req = self.set_headers(req);

        let req = req
            .header(ACCEPT_ENCODING, "gzip")
            .body(Body::empty())
            .map_err(|e| EurekaClientError::GenericError(e.to_string()))?;

        let res = self.client.request(req).await?;
        let status = res.status();

        debug!("get_application: server response {:?}", res);

        if status == StatusCode::NOT_FOUND {
            return Err(EurekaClientError::NotFound);
        }

        let body_bytes = body::to_bytes(res.into_body())
            .await
            .map_err(EurekaClientError::ClientError)?;

        let app: ApplicationResponse = serde_json::from_slice(&body_bytes)?;
        Ok(app)
    }

    pub async fn get_applications(&self) -> Result<ApplicationsResponse, EurekaClientError> {
        let path = "/v2/apps";
        let uri = self.build_uri(path)?;

        debug!("get_applications uri:{}", uri);

        let mut req = Request::builder().method(Method::GET).uri(uri);

        req = self.set_headers(req);

        let req = req
            .body(Body::empty())
            .map_err(|e| EurekaClientError::GenericError(e.to_string()))?;

        let res = self.client.request(req).await?;
        let status = res.status();

        debug!("get_applications: server response {:?}", res);

        if status == StatusCode::NOT_FOUND {
            debug!("received NotFound (404) from server");
            return Err(EurekaClientError::NotFound);
        }

        let body_bytes = body::to_bytes(res.into_body())
            .await
            .map_err(EurekaClientError::ClientError)?;

        let apps: ApplicationsResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
            warn!("serde error: {:?}", e);
            EurekaClientError::from(e)
        })?;

        debug!("returning: {:?}", apps);
        Ok(apps)
    }

    fn build_uri(&self, path: &str) -> Result<Uri, EurekaClientError> {
        let url = format!("{}{}", self.eureka_cluster_url, path);
        url.parse()
            .map_err(|e| EurekaClientError::GenericError(format!("Invalid URI: {}", e)))
    }

    fn set_headers(&self, builder: hyper::http::request::Builder) -> hyper::http::request::Builder {
        let user_agent = format!("Rust Hyper/{}", self.client_name);
        builder
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT_CHARSET, "utf-8")
            .header(USER_AGENT, user_agent)
    }
}
