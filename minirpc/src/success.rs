//! MINI-RPC Response Success.

/// Response success.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Success<R> {
    /// Correlation id.
    pub id: u64,

    /// Result.
    pub result: R,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    type R = bool;

    #[test]
    fn success_deserialization() {
        let input = r#"{"id":1,"result":true}"#;
        let expected = Success {
            id: 1,
            result: true,
        };

        let result: Success<R> = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn success_serialization() {
        let input = Success {
            id: 1,
            result: true,
        };
        let expected = r#"{"id":1,"result":true}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
