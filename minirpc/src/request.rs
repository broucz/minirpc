use crate::{Call, Notification};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Request {
    Batch(Vec<Payload>),
    Single(Payload),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Payload {
    Notification(Notification),
    Call(Call),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Call, Id, Method, Notification, Params};
    use serde_json::{self, Value};

    #[test]
    fn request_deserialization() {
        // Single Notification.
        let input = r#"{"method":"test_method","params":[1,2,3]}"#;
        let expected = Request::Single(Payload::Notification(Notification {
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        }));

        let result: Request = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Single Call.
        let input = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;
        let expected = Request::Single(Payload::Call(Call {
            id: Id::Number(1),
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        }));

        let result: Request = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Batch Notification and Call.
        let input =
            r#"[{"method":"test_method","params":[1,2,3]},{"id":1,"method":"test_method","params":[1,2,3]}]"#;
        let expected = Request::Batch(vec![
            Payload::Notification(Notification {
                method: Method::String("test_method".to_owned()),
                params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
            }),
            Payload::Call(Call {
                id: Id::Number(1),
                method: Method::String("test_method".to_owned()),
                params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
            }),
        ]);

        let result: Request = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn request_serialization() {
        // Single Notification.
        let input = Request::Single(Payload::Notification(Notification {
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        }));
        let expected = r#"{"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Single Call.
        let input = Request::Single(Payload::Call(Call {
            id: Id::Number(1),
            method: Method::String("test_method".to_owned()),
            params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
        }));
        let expected = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Batch Notification and Call.
        let input = Request::Batch(vec![
            Payload::Notification(Notification {
                method: Method::String("test_method".to_owned()),
                params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
            }),
            Payload::Call(Call {
                id: Id::Number(1),
                method: Method::String("test_method".to_owned()),
                params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
            }),
        ]);
        let expected =
            r#"[{"method":"test_method","params":[1,2,3]},{"id":1,"method":"test_method","params":[1,2,3]}]"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
