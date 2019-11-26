//! MINI-RPC is a stateless, light-weight remote procedure call (RPC) protocol.
//! Inspired by the [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification),
//! this implementation written in Rust selected its essential parts in order to keep
//! as minimal as possible the communication between processes.

#[macro_use]
extern crate serde_derive;

pub mod call;
pub mod error;
pub mod failure;
pub mod notification;
pub mod request;
pub mod response;
pub mod success;

pub use self::call::Call;
pub use self::error::Error;
pub use self::failure::Failure;
pub use self::notification::Notification;
pub use self::request::Payload as RequestPayload;
pub use self::request::Request;
pub use self::response::Payload as ResponsePayload;
pub use self::response::Response;
pub use self::success::Success;
pub use serde_json::{Map, Value};
