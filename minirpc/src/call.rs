//! MINI-RPC Call Request.

/// Represents a MINI-RPC Request which is a call.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Call<M, P> {
    /// Call request id.
    pub id: u64,

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
    fn call_deserialization() {
        let input = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;
        let expected = Call {
            id: 1,
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        };

        let result: Call<M, P> = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn call_serialization() {
        let input = Call {
            id: 1,
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        };
        let expected = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
