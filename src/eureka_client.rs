use crate::errors::EurekaClientError;
use crate::request::RegisterRequest;
use crate::response::{ApplicationResponse, ApplicationsResponse};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_CHARSET, ACCEPT_ENCODING, CONTENT_TYPE, USER_AGENT,
};
use reqwest::{Client, StatusCode, Url};
use serde_json;
use std::time::Duration;

/// A client for accessing Eureka
pub struct EurekaClient {
    client: Client,
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
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");
        EurekaClient {
            client,
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
        // build both /v2 and non-/v2 variants; build_uris will produce candidate URIs
        let _path = format!("/v2/apps/{}", application_id);

        // Build a conservative XML payload which many Eureka servers accept for registration
        let inst = &register_request.instance;
        let port_xml = if let Some(p) = inst.port {
            format!("<port enabled=\"true\">{}</port>", p)
        } else {
            String::new()
        };
        let secure_port_xml = if let Some(p) = inst.secure_port {
            format!("<securePort enabled=\"true\">{}</securePort>", p)
        } else {
            String::new()
        };
        let dci_xml = match inst.data_center_info.name {
            crate::request::DcName::MyOwn => "<dataCenterInfo class=\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\"><name>MyOwn</name></dataCenterInfo>".to_string(),
            crate::request::DcName::Amazon => "<dataCenterInfo class=\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\"><name>Amazon</name></dataCenterInfo>".to_string(),
        };
        let lease_xml = if let Some(li) = &inst.lease_info {
            let eviction = li.eviction_duration_in_secs.unwrap_or(90);
            format!(
                "<leaseInfo><evictionDurationInSecs>{}</evictionDurationInSecs></leaseInfo>",
                eviction
            )
        } else {
            String::new()
        };

        let xml = format!(
            "<instance>\n  <hostName>{}</hostName>\n  <app>{}</app>\n  <ipAddr>{}</ipAddr>\n  <vipAddress>{}</vipAddress>\n  <secureVipAddress>{}</secureVipAddress>\n  <status>{}</status>\n  {}\n  {}\n  <countryId>1</countryId>\n  {}\n  {}\n  <metadata class=\"java.util.Collections$EmptyMap\"/>\n  <homePageUrl>{}</homePageUrl>\n  <statusPageUrl>{}</statusPageUrl>\n  <healthCheckUrl>{}</healthCheckUrl>\n</instance>",
            inst.host_name,
            inst.app,
            inst.ip_addr,
            inst.vip_address,
            inst.secure_vip_address,
            String::from(&inst.status),
            port_xml,
            secure_port_xml,
            dci_xml,
            lease_xml,
            inst.homepage_url,
            inst.status_page_url,
            inst.health_check_url
        );

        // Prefer the /eureka/apps endpoint first (observed to succeed on Spring Cloud Eureka instances).
        let base = &self.eureka_cluster_url;
        // Prefer known working endpoints ordering (try v2 variant first)
        let candidates = vec![
            format!("{}/eureka/v2/apps/{}", base, application_id),
            format!("{}/eureka/apps/{}", base, application_id),
            format!("{}/v2/apps/{}", base, application_id),
            format!("{}/apps/{}", base, application_id),
        ];

        let mut last_err: Option<EurekaClientError> = None;
        let mut saw_bad_request = false;
        let mut saw_internal_server_error = false;

        // Try XML registration first (Spring Cloud Eureka often expects XML)
        for uri_str in &candidates {
            let url =
                Url::parse(uri_str).map_err(|e| EurekaClientError::GenericError(e.to_string()))?;
            let res = match self
                .client
                .post(url.clone())
                .header("content-type", "application/xml")
                .body(xml.clone())
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    last_err = Some(EurekaClientError::from(e));
                    continue;
                }
            };

            let status = res.status();
            let body_bytes = res.bytes().await.map_err(EurekaClientError::from)?;
            let _body_str = String::from_utf8_lossy(&body_bytes);

            if status.is_success() {
                return Ok(());
            }

            if status == StatusCode::BAD_REQUEST {
                saw_bad_request = true;
            } else if status == StatusCode::INTERNAL_SERVER_ERROR {
                saw_internal_server_error = true;
            }
        }

        // Fall back to JSON payload. Build a conservative JSON shape matching what worked via curl.
        let inst = &register_request.instance;
        let mut inst_map = serde_json::Map::new();
        inst_map.insert(
            "hostName".to_string(),
            serde_json::Value::String(inst.host_name.clone()),
        );
        inst_map.insert(
            "app".to_string(),
            serde_json::Value::String(inst.app.clone()),
        );
        inst_map.insert(
            "ipAddr".to_string(),
            serde_json::Value::String(inst.ip_addr.clone()),
        );
        inst_map.insert(
            "vipAddress".to_string(),
            serde_json::Value::String(inst.vip_address.clone()),
        );
        inst_map.insert(
            "secureVipAddress".to_string(),
            serde_json::Value::String(inst.secure_vip_address.clone()),
        );
        inst_map.insert(
            "status".to_string(),
            serde_json::Value::String(String::from(&inst.status)),
        );
        if let Some(p) = inst.port {
            let mut port_obj = serde_json::Map::new();
            port_obj.insert(
                "$".to_string(),
                serde_json::Value::Number(serde_json::Number::from(p)),
            );
            port_obj.insert(
                "@enabled".to_string(),
                serde_json::Value::String("true".to_string()),
            );
            inst_map.insert("port".to_string(), serde_json::Value::Object(port_obj));
        }
        if let Some(p) = inst.secure_port {
            let mut port_obj = serde_json::Map::new();
            port_obj.insert(
                "$".to_string(),
                serde_json::Value::Number(serde_json::Number::from(p)),
            );
            port_obj.insert(
                "@enabled".to_string(),
                serde_json::Value::String("false".to_string()),
            );
            inst_map.insert(
                "securePort".to_string(),
                serde_json::Value::Object(port_obj),
            );
        }
        inst_map.insert(
            "homePageUrl".to_string(),
            serde_json::Value::String(inst.homepage_url.clone()),
        );
        inst_map.insert(
            "statusPageUrl".to_string(),
            serde_json::Value::String(inst.status_page_url.clone()),
        );
        inst_map.insert(
            "healthCheckUrl".to_string(),
            serde_json::Value::String(inst.health_check_url.clone()),
        );
        // dataCenterInfo with @class and name
        let mut dci = serde_json::Map::new();
        dci.insert(
            "@class".to_string(),
            serde_json::Value::String(
                "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo".to_string(),
            ),
        );
        dci.insert(
            "name".to_string(),
            serde_json::Value::String(match inst.data_center_info.name {
                crate::request::DcName::MyOwn => "MyOwn".to_string(),
                crate::request::DcName::Amazon => "Amazon".to_string(),
            }),
        );
        inst_map.insert("dataCenterInfo".to_string(), serde_json::Value::Object(dci));

        // lease
        if let Some(li) = &inst.lease_info {
            let mut lease = serde_json::Map::new();
            lease.insert(
                "evictionDurationInSecs".to_string(),
                serde_json::Value::Number(serde_json::Number::from(
                    li.eviction_duration_in_secs.unwrap_or(90),
                )),
            );
            inst_map.insert("leaseInfo".to_string(), serde_json::Value::Object(lease));
        }

        // metadata (top-level) - send Java empty map marker
        let mut meta_obj = serde_json::Map::new();
        meta_obj.insert(
            "@class".to_string(),
            serde_json::Value::String("java.util.Collections$EmptyMap".to_string()),
        );
        inst_map.insert("metadata".to_string(), serde_json::Value::Object(meta_obj));
        // countryId
        inst_map.insert(
            "countryId".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1)),
        );

        let mut top = serde_json::Map::new();
        top.insert("instance".to_string(), serde_json::Value::Object(inst_map));
        let manual_json = serde_json::Value::Object(top).to_string();

        for uri_str in &candidates {
            let url =
                Url::parse(uri_str).map_err(|e| EurekaClientError::GenericError(e.to_string()))?;
            let req_builder = self.client.post(url.clone());
            // apply headers that the original code used
            let req_builder = req_builder
                .header("accept", "application/json")
                .header("content-type", "application/json;charset=UTF-8")
                .header("accept-charset", "utf-8")
                .header("user-agent", format!("Rust Reqwest/{}", self.client_name));

            let res = match req_builder.body(manual_json.clone()).send().await {
                Ok(r) => r,
                Err(e) => {
                    last_err = Some(EurekaClientError::from(e));
                    continue;
                }
            };

            let status = res.status();
            let body_bytes = res.bytes().await.map_err(EurekaClientError::from)?;
            let _body_str = String::from_utf8_lossy(&body_bytes);

            match status {
                StatusCode::BAD_REQUEST => {
                    saw_bad_request = true;
                    continue;
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    saw_internal_server_error = true;
                    continue;
                }
                StatusCode::NOT_FOUND => {
                    // Try next URI
                    continue;
                }
                _ => return Ok(()),
            }
        }

        // If we exhausted URIs decide which error to return
        if let Some(e) = last_err {
            Err(e)
        } else if saw_internal_server_error {
            Err(EurekaClientError::InternalServerError)
        } else if saw_bad_request {
            Err(EurekaClientError::BadRequest)
        } else {
            Err(EurekaClientError::NotFound)
        }
    }

    pub async fn deregister(
        &self,
        application_id: &str,
        instance_id: &str,
    ) -> Result<(), EurekaClientError> {
        // Build base path for application: /v2/apps/{app}
        let base_path = format!("/v2/apps/{}", application_id);

        let app_uris = self.build_uris(&base_path)?; // returns Vec<Url> pointing to .../apps/{app}
        let mut last_err: Option<EurekaClientError> = None;
        let mut saw_bad_request = false;
        let mut saw_internal_server_error = false;

        for app_url in app_uris {
            // Attempt to append the instance_id as a path segment using Url::join which will
            // percent-encode characters like ':' appropriately.
            // Build the delete URL by appending the instance_id as a path segment.
            // Use path_segments_mut to ensure proper percent-encoding of characters like ':'.
            let mut delete_url = app_url.clone();
            match delete_url.path_segments_mut() {
                Ok(mut segments) => {
                    segments.push(instance_id);
                }
                Err(e) => {
                    last_err = Some(EurekaClientError::GenericError(format!(
                        "Failed to append instance id to URL {e:?}"
                    )));
                    continue;
                }
            }

            let headers = self.headers_map();
            let res = match self
                .client
                .delete(delete_url.clone())
                .headers(headers.clone())
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    last_err = Some(EurekaClientError::from(e));
                    continue;
                }
            };

            let status = res.status();

            debug!(
                "deregister: server response status={:?} for url={} ",
                status, delete_url
            );

            if status.is_success() {
                return Ok(());
            }

            if status == StatusCode::BAD_REQUEST {
                saw_bad_request = true;
                continue;
            } else if status == StatusCode::INTERNAL_SERVER_ERROR {
                saw_internal_server_error = true;
                continue;
            } else if status == StatusCode::NOT_FOUND {
                // try next URI
                continue;
            }
        }

        // If we exhausted URIs decide which error to return
        if let Some(e) = last_err {
            Err(e)
        } else if saw_internal_server_error {
            Err(EurekaClientError::InternalServerError)
        } else if saw_bad_request {
            Err(EurekaClientError::BadRequest)
        } else {
            Err(EurekaClientError::NotFound)
        }
    }

    pub async fn get_application(
        &self,
        application_id: &str,
    ) -> Result<ApplicationResponse, EurekaClientError> {
        let _path = format!("/v2/apps/{}", application_id);

        let uris = self.build_uris(&_path)?; // returns Vec<Url>
        let mut last_err: Option<EurekaClientError> = None;

        for url in uris {
            let headers = self.headers_map();
            let res = match self
                .client
                .get(url.clone())
                .headers(headers.clone())
                .header(ACCEPT_ENCODING, "gzip")
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    last_err = Some(EurekaClientError::from(e));
                    continue;
                }
            };

            let status = res.status();

            debug!("get_application: server response status={:?}", status);

            if status == StatusCode::NOT_FOUND {
                // try next URI
                continue;
            }

            let body_bytes = res.bytes().await.map_err(EurekaClientError::from)?;

            let app: ApplicationResponse = serde_json::from_slice(&body_bytes)?;
            return Ok(app);
        }

        if let Some(e) = last_err {
            Err(e)
        } else {
            Err(EurekaClientError::NotFound)
        }
    }

    pub async fn get_applications(&self) -> Result<ApplicationsResponse, EurekaClientError> {
        let path = "/v2/apps";

        let uris = self.build_uris(path)?;
        let mut last_err: Option<EurekaClientError> = None;

        for url in uris {
            debug!("get_applications url:{}", url);

            let headers = self.headers_map();
            let res = match self
                .client
                .get(url.clone())
                .headers(headers.clone())
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    last_err = Some(EurekaClientError::from(e));
                    continue;
                }
            };
            let status = res.status();

            debug!("get_applications: server response status={:?}", status);

            if status == StatusCode::NOT_FOUND {
                debug!("received NotFound (404) from server");
                continue;
            }

            let body_bytes = res.bytes().await.map_err(EurekaClientError::from)?;

            let apps: ApplicationsResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
                warn!("serde error: {:?}", e);
                EurekaClientError::from(e)
            })?;

            debug!("returning: {:?}", apps);
            return Ok(apps);
        }

        if let Some(e) = last_err {
            Err(e)
        } else {
            Err(EurekaClientError::NotFound)
        }
    }

    fn build_uri(&self, path: &str) -> Result<Url, EurekaClientError> {
        let url = format!("{}{}", self.eureka_cluster_url, path);
        Url::parse(&url).map_err(|e| EurekaClientError::GenericError(format!("Invalid URI: {}", e)))
    }

    /// Build a list of candidate URIs to try for a given path.
    /// Some Eureka distributions mount under /eureka, others serve at root. To be robust,
    /// we try both the configured base URL as-is and with a `/eureka` prefix when appropriate.
    fn build_uris(&self, path: &str) -> Result<Vec<Url>, EurekaClientError> {
        let mut uris = Vec::new();

        // candidate paths: original and legacy without /v2 prefix
        let mut paths = vec![path.to_string()];
        if path.starts_with("/v2") {
            paths.push(path.replacen("/v2", "", 1));
        }

        let prefix = "/eureka";

        for p in paths {
            // direct
            if let Ok(u) = self.build_uri(&p) {
                if !uris.contains(&u) {
                    uris.push(u);
                }
            }
            // try with /eureka prefix if not already present in base
            if !self.eureka_cluster_url.ends_with(prefix) {
                let path_with_prefix = format!("{}{}", prefix, p);
                if let Ok(u) = self.build_uri(&path_with_prefix) {
                    if !uris.contains(&u) {
                        uris.push(u);
                    }
                }
            }
        }

        if uris.is_empty() {
            return Err(EurekaClientError::GenericError("No valid URIs".to_string()));
        }

        Ok(uris)
    }

    fn headers_map(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json;charset=UTF-8"),
        );
        headers.insert(ACCEPT_CHARSET, HeaderValue::from_static("utf-8"));
        let ua = HeaderValue::from_str(&format!("Rust Reqwest/{}", self.client_name))
            .unwrap_or_else(|_| HeaderValue::from_static("Rust Reqwest"));
        headers.insert(USER_AGENT, ua);
        headers
    }
}
