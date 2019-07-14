//! MINI-RPC Request Parameters.

use serde_json::{Map, Value};

/// Request parameters.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Params {
    /// Array of values.
    Array(Vec<Value>),

    /// Map of values.
    Object(Map<String, Value>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn params_deserialization() {
        // Array.
        let input = r#"[1,true]"#;
        let expected = Params::Array(vec![Value::from(1), Value::Bool(true)]);

        let result: Params = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Object.
        let input = r#"{"foo":"bar"}"#;
        let mut map = Map::new();
        map.insert("foo".to_string(), "bar".into());
        let expected = Params::Object(map);

        let result: Params = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn params_serialization() {
        // Array.
        let input = Params::Array(vec![Value::from(1), Value::Bool(true)]);
        let expected = r#"[1,true]"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Object.
        let mut input = Map::new();
        input.insert("foo".to_string(), "bar".into());
        let expected = r#"{"foo":"bar"}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
