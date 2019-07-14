//! MINI-RPC Response Failure.

use crate::{Error, Id};

/// Response failure.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Failure {
    /// Error.
    pub error: Error,

    /// Correlation id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn failure_deserialization() {
        let input = r#"{"error":{"code":-32700,"message":"Parse error"},"id":1}"#;
        let expected = Failure {
            error: Error::new_parse_error(),
            id: Some(Id::Number(1)),
        };

        let result: Failure = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn failure_serialization() {
        let input = Failure {
            error: Error::new_parse_error(),
            id: Some(Id::Number(1)),
        };
        let expected = r#"{"error":{"code":-32700,"message":"Parse error"},"id":1}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
