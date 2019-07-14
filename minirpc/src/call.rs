//! MINI-RPC Call Request.

use crate::{Id, Method, Params};

/// Represents a MINI-RPC Request which is a call.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    /// Call request id.
    pub id: Id,

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
    fn call_deserialization() {
        let input = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;
        let expected = Call {
            id: Id::Number(1),
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        };

        let result: Call = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn call_serialization() {
        let input = Call {
            id: Id::Number(1),
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        };
        let expected = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
