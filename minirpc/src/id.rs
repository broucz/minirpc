//! MINI-RPC Id.

/// Request id.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    /// Numeric id.
    Number(u64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn id_deserialization() {
        let input = r#"1"#;
        let expected = Id::Number(1);

        let result: Id = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn id_serialization() {
        let input = Id::Number(1);
        let expected = r#"1"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
