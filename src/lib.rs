#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate url;
#[macro_use]
extern crate log;
extern crate option_filter;

mod errors;
mod http;

pub mod eureka_client;
pub mod model;
