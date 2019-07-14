//! MINI-RPC Notification Request.

use crate::{Method, Params};

/// Represents a MINI-RPC Request which is a notification.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
    /// A String containing the name of the method to be invoked.
    pub method: Method,

    /// A Structured value that holds the parameter values to be used during the invocation of the method.
    pub params: Params,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, Value};

    #[test]
    fn notification_deserialization() {
        let input = r#"{"method":"test_method","params":[1,2,3]}"#;
        let expected = Notification {
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        };

        let result: Notification = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn notification_serialization() {
        let input = Notification {
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        };
        let expected = r#"{"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
