//! MINI-RPC Notification Request.

/// Represents a MINI-RPC Request which is a notification.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Notification<M, P> {
    /// A String containing the name of the method to be invoked.
    pub method: M,

    /// A Structured value that holds the parameter values to be used during the invocation of the method.
    pub params: P,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    type M = String;
    type P = Vec<u64>;

    #[test]
    fn notification_deserialization() {
        let input = r#"{"method":"test_method","params":[1,2,3]}"#;
        let expected = Notification {
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        };

        let result: Notification<M, P> = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn notification_serialization() {
        let input = Notification {
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        };
        let expected = r#"{"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
