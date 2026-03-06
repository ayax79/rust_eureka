use hyper::http::uri::InvalidUri;
use serde_json::error::Error as ParserError;
use std::error::Error;
use std::fmt::{self, Display};

use self::EurekaClientError::*;

/// Errors that can be returned by the [EurekaClient](struct.EurekaClient.html)
#[derive(Debug)]
pub enum EurekaClientError {
    /// An underlying error occurred with the Hyper http client
    ClientError(hyper::Error),
    /// An error occurred parsing a response from the server
    JsonError(ParserError),
    /// A generic error that was no otherwise typed occurred
    GenericError(String),
    /// The Uri of the Eureka server was invalid
    InvalidUri(InvalidUri),
    /// An server error occurred with Eureka
    InternalServerError,
    /// Request parameters sent to Eureka were invalid
    BadRequest,
    /// The specified resource does not exist in eureka, such as an invalid application name
    NotFound,
}

impl Error for EurekaClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ClientError(ref error) => Some(error),
            JsonError(ref error) => Some(error),
            InvalidUri(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<hyper::Error> for EurekaClientError {
    fn from(err: hyper::Error) -> EurekaClientError {
        ClientError(err)
    }
}

impl From<ParserError> for EurekaClientError {
    fn from(err: ParserError) -> EurekaClientError {
        JsonError(err)
    }
}

impl From<InvalidUri> for EurekaClientError {
    fn from(err: InvalidUri) -> EurekaClientError {
        InvalidUri(err)
    }
}

impl Display for EurekaClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError(e) => write!(f, "HTTP client error: {}", e),
            JsonError(e) => write!(f, "JSON parsing error: {}", e),
            GenericError(s) => write!(f, "Generic error: {}", s),
            InvalidUri(e) => write!(f, "Invalid URI: {}", e),
            InternalServerError => write!(f, "Internal server error (500)"),
            BadRequest => write!(f, "Bad request (400)"),
            NotFound => write!(f, "Not found (404)"),
        }
    }
}
