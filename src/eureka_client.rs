use std::io;
use futures::{Future, Stream};
use serde_json;
use model::Instance;
use errors::EurekaClientError;
use hyper::{Client, Method, Request, Body, Uri, mime, Error as HyperError};
use hyper::header::{AcceptEncoding, Encoding, UserAgent, ContentType, qitem};
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

    pub fn get_application_instances<'b>(&self, application_id: &str) -> impl Future<Item = Vec<Instance>, Error = EurekaClientError> {
        let client = Client::new(self.handle);
        let user_agent = "Rust Hyper/".to_string() + self.client_name.as_ref();

        let uri: Uri = (self.eureka_cluster_url.to_owned() + "/eureka/v2/apps/" + application_id).parse().unwrap();
        let mut req: Request<Body> = Request::new(Method::Post, uri);
        req.headers_mut().set(AcceptEncoding(vec![qitem(Encoding::Gzip)]));
        req.headers_mut().set(ContentType(mime::APPLICATION_JSON));
        req.headers_mut().set(UserAgent::new(user_agent));

        client.request(req).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                serde_json::from_slice::<Vec<Instance>>(&body).map_err(|e| {
                    HyperError::Io(io::Error::new(io::ErrorKind::Other, e))
                })
            })
        }).map_err(|e| {
            EurekaClientError::ClientError(e)
        })
    }
}
