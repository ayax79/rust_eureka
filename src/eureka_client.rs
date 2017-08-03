use std::io;
use futures::{Future, Stream};
use serde_json;
use model::Instance;
use errors::EurekaClientError;
use hyper::{Client, Method, Request, Body, Uri, mime, Error as HyperError};
use hyper::header::{Accept, AcceptEncoding, Encoding, Headers, UserAgent, ContentType, ContentLength, qitem};
use tokio_core::reactor::Handle;

pub struct EurekaClient<'a> {
    handle: &'a Handle,
    client_name: String,
    eureka_cluster_url: String,
}

//
// A simple port of the example found at: https://github.com/Netflix/eureka/wiki/Example-Custom-ReadOnly-client
// Eureka REST API: https://github.com/Netflix/eureka/wiki/Eureka-REST-operations
impl<'a> EurekaClient<'a> {
    pub fn new(handle: &'a Handle, client_name: &str, eureka_cluster_url: &str) -> EurekaClient<'a> {
        EurekaClient {
            handle: &handle,
            client_name: client_name.to_owned(),
            eureka_cluster_url: eureka_cluster_url.to_owned()
        }
    }

    pub fn register(&self, application_id: &str, instance: &Instance) -> impl Future<Item=(), Error=EurekaClientError> {
        let client = Client::new(self.handle);
        let path = "/eureka/v2/apps/".to_owned() + application_id;
        let mut req: Request<Body> = Request::new(Method::Post, self.build_uri(path.as_ref()));
        self.set_headers(req.headers_mut());

        let json = serde_json::to_string(instance).unwrap();
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        client.request(req).and_then(|res| {
            debug!("get_application_instance: server response status: {}", res.status());
            Ok(())
        }).map_err(|e| {
            EurekaClientError::ClientError(e)
        })
    }

    pub fn get_application_instances<'b>(&self, application_id: &str) -> impl Future<Item=Vec<Instance>, Error=EurekaClientError> {
        let client = Client::new(self.handle);
        let path = "/eureka/v2/apps/".to_owned() + application_id;
        let mut req: Request<Body> = Request::new(Method::Get, self.build_uri(path.as_ref()));
        self.set_headers(req.headers_mut());

        client.request(req).and_then(|res| {
            debug!("get_application_instance: server response status: {}", res.status());
            res.body().concat2().and_then(move |body| {
                serde_json::from_slice::<Vec<Instance>>(&body).map_err(|e| {
                    HyperError::Io(io::Error::new(io::ErrorKind::Other, e))
                })
            })
        }).map_err(|e| {
            EurekaClientError::ClientError(e)
        })
    }

    fn build_uri(&self, path: &str) -> Uri {
        (self.eureka_cluster_url.to_owned() + path).parse().unwrap()
    }

    fn set_headers(&self, headers: &mut Headers) {
        headers.set(AcceptEncoding(vec![qitem(Encoding::Gzip)]));
        headers.set(Accept(vec![qitem(mime::APPLICATION_JSON)]));
        headers.set(ContentType(mime::APPLICATION_JSON));
        let user_agent = "Rust Hyper/".to_string() + self.client_name.as_ref();
        headers.set(UserAgent::new(user_agent));
    }
}
