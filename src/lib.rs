extern crate base64;
extern crate hyper;
extern crate iso8601;
extern crate xml;

mod error;
mod parser;
mod request;
mod value;

pub use error::{RequestError, Fault};
pub use request::{Request, RequestResult};
pub use value::Value;

/// A response from the server.
///
/// XML-RPC specifies that a call should either return a single `Value`, or a `<fault>`.
pub type Response = Result<Value, Fault>;
