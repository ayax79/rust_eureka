use std::error::Error;
use std::fmt::Display;
use std::fmt;
use std::convert::From;
use hyper::error::Error as HyperError;
use serde_json::error::Error as ParserError;
use hyper::error::UriError;

use self::EurekaClientError::*;

/// Errors that can be returned by the [EurekaClient](struct.EurekaClient.html)
#[derive(Debug)]
pub enum EurekaClientError {
    /// An underlying error occurred with the Hyper http client
    ClientError(HyperError),
    /// An error occurred parsing a response from the server
    JsonError(ParserError),
    /// A generic error that was no otherwise typed occurred
    GenericError(String),
    /// The Uri of the Eureka server was invalid
    InvalidUri(UriError),
    /// An server error occurred with Eureka
    InternalServerError,
    /// Request parameters sent to Eureka were invalid
    BadRequest,
    /// The specified resource does not exist in eureka, such as an invalid application name
    NotFound
}

impl Error for EurekaClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError(_) => "Error calling downstream client: ",
            JsonError(_) => "A json error occurred ",
            BadRequest => "Received a 400 (Bad Request) response",
            _ => "Some error occurred"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ClientError(ref error) => Some(error as &Error),
            JsonError(ref error) => Some(error as &Error),
            _ => None
        }
    }
}

impl From<HyperError> for EurekaClientError {
    fn from(err: HyperError) -> EurekaClientError {
        ClientError(err)
    }
}

impl From<ParserError> for EurekaClientError {
    fn from(err: ParserError) -> EurekaClientError {
        JsonError(err)
    }
}

impl From<UriError> for EurekaClientError {
    fn from(err: UriError) -> EurekaClientError {
        InvalidUri(err)
    }
}

impl Display for EurekaClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}