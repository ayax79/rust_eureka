use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, Accept, Connection, AcceptEncoding, Encoding, UserAgent, ContentType};
use hyper::header::{ContentLength, qitem};
use hyper::client::Response;
use hyper::mime;
use errors::EurekaClientError;
use errors::EurekaClientError::GenericError;
use tokio_core::reactor::Handle;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use hyper::{Method, Request};


fn foo() {

}

//pub fn http_get(handle: &Handle, client_name: &str, url: &str) -> Result<String, EurekaClientError> {
//    // println!("URL: {:?} HEADERS: {:?}", url, maybe_headers);
//    let client = Client::new(handle);
//    let user_agent = "Rust Hyper/".to_string() + client_name;
//
//    let uri = url.parse()?;
//    let mut req = Request::new(Method::Post, uri);
//    req.headers_mut().set(AcceptEncoding(vec![qitem(Encoding::Gzip)]));
//    req.headers_mut().set(ContentType(mime::APPLICATION_JSON));
//    req.headers_mut().set(UserAgent::new(user_agent));
//
//    client.request(uri).and_then(|res| {
//
//
//            .and_teh(read_response)
//            .and_then(|mut res| read_response(&mut res))
//
//    })
//        .map_err(|err| EurekaClientError::from(err))
//
//}
//
////pub fn parse_json(raw_report: &String) -> Result<Value, EurekaClientError> {
////    info!("raw {:?}", raw_report);
////    Json::from_str(&raw_report).map_err(|err| ReportError::from(err))
////}
//
//// reads the response into a string
//fn read_response(res: &mut Response) -> Future<String, EurekaClientError> {
//    if !res.status().is_success() {
//        error!("error response code: {:?} res: {:?}", res.status, res);
//        Err(GenericError("Request was not successful".to_string()))
//    }
//        else {
//            let mut body = String::new();
//            res.read_to_string(&mut body).unwrap();
//            info!("\nread_response: body {:?}", body);
//            Ok(body)
//        }
//}
//
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_build_headers() {
//        let headers = build_headers("foo");
////        headers.has::
//
//
//    }
//
//
//}