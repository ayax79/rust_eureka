use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, Accept, Connection, AcceptEncoding, Encoding, UserAgent, qitem};
use hyper::client::Response;
use errors::EurekaClientError;
use errors::EurekaClientError::GenericError;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub fn http_get(client_name: &str, url: &str) -> Result<String, EurekaClientError> {
    // println!("URL: {:?} HEADERS: {:?}", url, maybe_headers);
    let headers = build_headers(client_name);
    let client = Client::new();
    client.get(url)
        .header(Connection::close())
        .headers(headers)
        .send()
        .map_err(|err| EurekaClientError::from(err))
        .and_then(|mut res| read_response(&mut res))
}

//pub fn parse_json(raw_report: &String) -> Result<Value, EurekaClientError> {
//    info!("raw {:?}", raw_report);
//    Json::from_str(&raw_report).map_err(|err| ReportError::from(err))
//}

// reads the response into a string
fn read_response(res: &mut Response) -> Result<String, EurekaClientError> {
    if !res.status.is_success() {
        error!("error response code: {:?} res: {:?}", res.status, res);
        Err(GenericError("Request was not successful".to_string()))
    }
        else {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            info!("\nread_response: body {:?}", body);
            Ok(body)
        }
}

fn build_headers(client_name: &str) -> Headers {
    let mut headers = Headers::new();
    let user_agent = "Rust Hyper/".to_string() + client_name;
    headers.set(Accept(vec![
        qitem(Mime(TopLevel::Application, SubLevel::Json,
                   vec![(Attr::Charset, Value::Utf8)])),
    ]));
    headers.set(AcceptEncoding(vec![
            qitem(Encoding::Gzip)
        ]));
    headers.set(UserAgent(user_agent));
    headers
}
