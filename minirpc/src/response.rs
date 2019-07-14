//! MINI-RPC Response.

use crate::{Failure, Success};

/// Response payload.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Payload {
    /// Unsuccessful response.
    Failure(Failure),

    /// Successful response.
    Success(Success),
}

/// Response.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Response {
    /// A batch of responses (payloads).
    Batch(Vec<Payload>),

    /// A single response (payload).
    Single(Payload),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Id};
    use serde_json::{self, Value};

    #[test]
    fn response_deserialization() {
        // Single Failure.
        let input = r#"{"error":{"code":-32700,"message":"Parse error"},"id":1}"#;
        let expected = Response::Single(Payload::Failure(Failure {
            error: Error::new_parse_error(),
            id: Some(Id::Number(1)),
        }));

        let result: Response = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Single Success.
        let input = r#"{"id":1,"result":true}"#;
        let expected = Response::Single(Payload::Success(Success {
            id: Id::Number(1),
            result: Value::Bool(true),
        }));

        let result: Response = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Batch Failure and Success.
        let input =
            r#"[{"error":{"code":-32700,"message":"Parse error"},"id":1},{"id":1,"result":true}]"#;
        let expected = Response::Batch(vec![
            Payload::Failure(Failure {
                error: Error::new_parse_error(),
                id: Some(Id::Number(1)),
            }),
            Payload::Success(Success {
                id: Id::Number(1),
                result: Value::Bool(true),
            }),
        ]);

        let result: Response = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn response_serialization() {
        // Single Failure.
        let input = Response::Single(Payload::Failure(Failure {
            error: Error::new_parse_error(),
            id: Some(Id::Number(1)),
        }));
        let expected = r#"{"error":{"code":-32700,"message":"Parse error"},"id":1}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Single Success.
        let input = Response::Single(Payload::Success(Success {
            id: Id::Number(1),
            result: Value::Bool(true),
        }));
        let expected = r#"{"id":1,"result":true}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Batch Failure and Success.
        let input = Response::Batch(vec![
            Payload::Failure(Failure {
                error: Error::new_parse_error(),
                id: Some(Id::Number(1)),
            }),
            Payload::Success(Success {
                id: Id::Number(1),
                result: Value::Bool(true),
            }),
        ]);
        let expected =
            r#"[{"error":{"code":-32700,"message":"Parse error"},"id":1},{"id":1,"result":true}]"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
