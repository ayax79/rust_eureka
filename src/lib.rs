#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate url;
//#[macro_use]
extern crate log;
extern crate option_filter;

mod errors;
mod http;

pub mod eureka_client;
pub mod model;
