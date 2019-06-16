use std::fmt;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Method {
    String(String),
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
           Method::String(string) => write!(f, "{}", string),
       }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn method_deserialization() {
        let input = r#""text_method""#;
        let expected = Method::String("text_method".to_owned());

        let result: Method = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn method_serialization() {
        let input = Method::String("text_method".to_owned());
        let expected = r#""text_method""#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
