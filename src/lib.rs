#[macro_use]
extern crate log;

pub mod errors;
pub mod eureka_client;
pub mod request;
pub mod response;

pub use eureka_client::EurekaClient;
