//! MINI-RPC Errors.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::fmt;

/// Error code.
#[derive(Debug, PartialEq)]
pub enum Code {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError,

    /// The JSON sent is not a valid Request object.
    InvalidRequest,

    /// The method does not exist / is not available.
    MethodNotFound,

    /// Invalid method parameter(s).
    InvalidParams,

    /// Internal MINI-RPC error.
    InternalError,

    /// Reserved for implementation-defined server-errors.
    ServerError(i64),
}

impl Code {
    pub fn message(&self) -> &str {
        match *self {
            Code::ParseError => "Parse error",
            Code::InvalidRequest => "Invalid request",
            Code::MethodNotFound => "Method not found",
            Code::InvalidParams => "Invalid params",
            Code::InternalError => "Internal error",
            Code::ServerError(_) => "Server error",
        }
    }
}

impl<'a> Deserialize<'a> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Code, D::Error>
    where
        D: Deserializer<'a>,
    {
        Ok(match i64::deserialize(deserializer)? {
            -32700 => Code::ParseError,
            -32600 => Code::InvalidRequest,
            -32601 => Code::MethodNotFound,
            -32602 => Code::InvalidParams,
            -32603 => Code::InternalError,
            code => Code::ServerError(code),
        })
    }
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(match *self {
            Code::ParseError => -32700,
            Code::InvalidRequest => -32600,
            Code::MethodNotFound => -32601,
            Code::InvalidParams => -32602,
            Code::InternalError => -32603,
            Code::ServerError(code) => code,
        })
    }
}

/// Error Object.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: Code,
    pub message: String,
}

impl Error {
    /// Creates a new `Error` for given `code`.
    pub fn new(code: Code) -> Self {
        let message = code.message().to_owned();
        Self { code, message }
    }

    /// Creates a new `ParseError`.
    pub fn new_parse_error() -> Self {
        Self::new(Code::ParseError)
    }

    /// Creates a new `ParseError`.
    pub fn new_invalid_request() -> Self {
        Self::new(Code::InvalidRequest)
    }

    /// Creates a new `MethodNotFound`.
    pub fn new_method_not_found() -> Self {
        Self::new(Code::MethodNotFound)
    }

    /// Creates a new `InvalidParams`.
    pub fn new_invalid_params() -> Self {
        Self::new(Code::InvalidParams)
    }

    /// Creates a new `InternalError`.
    pub fn new_internal_error() -> Self {
        Self::new(Code::InternalError)
    }

    /// Creates a new `ServerError` for given `code` and `message`.
    pub fn new_server_error(code: i64, message: &str) -> Self {
        Self {
            code: Code::ServerError(code),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn code_deserialization() {
        let input = r#"-32700"#;
        let expected = Code::ParseError;

        let result: Code = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn code_serialization() {
        let input = Code::ParseError;
        let expected = r#"-32700"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn error_deserialization() {
        let input = r#"{"code":-32700,"message":"Parse error"}"#;
        let expected = Error::new(Code::ParseError);

        let result: Error = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn error_serialization() {
        let input = Error::new(Code::ParseError);
        let expected = r#"{"code":-32700,"message":"Parse error"}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn error_new() {
        let result = Error::new(Code::ParseError);
        let expected = Error {
            code: Code::ParseError,
            message: "Parse error".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn error_new_parse_error() {
        let result = Error::new_parse_error();
        let expected = Error {
            code: Code::ParseError,
            message: "Parse error".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn error_new_invalid_request() {
        let result = Error::new_invalid_request();
        let expected = Error {
            code: Code::InvalidRequest,
            message: "Invalid request".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn error_new_method_not_found() {
        let result = Error::new_method_not_found();
        let expected = Error {
            code: Code::MethodNotFound,
            message: "Method not found".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn error_new_invalid_params() {
        let result = Error::new_invalid_params();
        let expected = Error {
            code: Code::InvalidParams,
            message: "Invalid params".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn error_new_internal_error() {
        let result = Error::new_internal_error();
        let expected = Error {
            code: Code::InternalError,
            message: "Internal error".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn error_new_server_error() {
        let result = Error::new_server_error(-32000, "Test error");
        let expected = Error {
            code: Code::ServerError(-32000),
            message: "Test error".to_owned(),
        };

        assert_eq!(result, expected);
    }
}
