//! MINI-RPC Response Success.

use crate::Id;
use serde_json::Value;

/// Response success.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Success {
    /// Correlation id.
    pub id: Id,

    /// Result.
    pub result: Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, Value};

    #[test]
    fn success_deserialization() {
        let input = r#"{"id":1,"result":true}"#;
        let expected = Success {
            id: Id::Number(1),
            result: Value::Bool(true),
        };

        let result: Success = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn success_serialization() {
        let input = Success {
            id: Id::Number(1),
            result: Value::Bool(true),
        };
        let expected = r#"{"id":1,"result":true}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
