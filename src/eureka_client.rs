use std::io;
use futures::{Future, Stream};
use serde_json;
use model::{RegisterRequest, Instance};
use errors::EurekaClientError;
use hyper::{Client, Method, Request, Body, Uri, mime, Error as HyperError, StatusCode};
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
        debug!("Creating new Eureka Client client_name:{:?}, eureka_client:{:?}", client_name, eureka_cluster_url);
        EurekaClient {
            handle: &handle,
            client_name: client_name.to_owned(),
            eureka_cluster_url: eureka_cluster_url.to_owned()
        }
    }

    pub fn register(&self, application_id: &str, register_request: &RegisterRequest) -> Box<Future<Item=(), Error=EurekaClientError>> {
        debug!("register: application_id={:?}, register_request:{:?}", application_id, register_request);
        let client = Client::new(self.handle);
        let path = "/v2/apps/".to_owned() + application_id;
        let mut req: Request<Body> = Request::new(Method::Post, self.build_uri(path.as_ref()));
        self.set_headers(req.headers_mut());

        let json = serde_json::to_string(register_request).unwrap();
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let result = client.request(req)
            .map_err(|e| {
                EurekaClientError::from(e)
            })
            .and_then(|res| {
                debug!("register: server response {:?}", res);

                let status = res.status();
                match status {
                    StatusCode::BadRequest => Err(EurekaClientError::BadRequest),
                    StatusCode::InternalServerError => Err(EurekaClientError::InternalServerError),
                    _ => Ok(())
                }
            });
        Box::new(result)
    }

    pub fn get_application_instances<'b>(&self, application_id: &str) -> Box<Future<Item=Vec<Instance>, Error=EurekaClientError>> {

        // Since it was hard to coerce the errot type into a EurekaClientError
        // I set the result in a holder then map result into an error or ok
        // There has to be a better way.. but this works.
        enum IntermediateResult {
            Ok(Vec<Instance>),
            Err(EurekaClientError)
        }

        let client = Client::new(self.handle);
        let path = "/v2/apps/".to_owned() + application_id;
        let mut req: Request<Body> = Request::new(Method::Get, self.build_uri(path.as_ref()));
        self.set_headers(req.headers_mut());

        let result = client.request(req).and_then(|res| {
            let status = res.status();
            debug!("get_application_instance: server response status: {}", status);
            res.body().concat2().and_then(move |body| {
                match status {
                    StatusCode::NotFound => Ok(IntermediateResult::Err(EurekaClientError::NotFound)),
                    _ => {
                        serde_json::from_slice::<Vec<Instance>>(&body).map_err(|e| {
                            HyperError::Io(io::Error::new(io::ErrorKind::Other, e))
                        })
                        .map(|r| IntermediateResult::Ok(r))
                    }
                }
            })
        })
        .map_err(|e| {
            EurekaClientError::from(e)
        })
        .and_then(|ir| {
            // now that we have changed the error to EurekaClientError
            // we can map our err back in
            match ir {
                IntermediateResult::Ok(vec) => Ok(vec),
                IntermediateResult::Err(err) => Err(err)
            }
        });
        Box::new(result)
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
