//! MINI-RPC Request.

use crate::{Call, Notification};

/// Request.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Request<M, P> {
    /// A batch of requests (payloads).
    Batch(Vec<Payload<M, P>>),

    /// A single request (payload).
    Single(Payload<M, P>),
}

/// Request payload.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Payload<M, P> {
    /// Fire a notification.
    Notification(Notification<M, P>),

    /// Call a method.
    Call(Call<M, P>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Call, Notification};
    use serde_json;

    type M = String;
    type P = Vec<u64>;

    #[test]
    fn request_deserialization() {
        // Single Notification.
        let input = r#"{"method":"test_method","params":[1,2,3]}"#;
        let expected = Request::Single(Payload::Notification(Notification {
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        }));

        let result: Request<M, P> = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Single Call.
        let input = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;
        let expected = Request::Single(Payload::Call(Call {
            id: 1,
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        }));

        let result: Request<M, P> = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Batch Notification and Call.
        let input =
            r#"[{"method":"test_method","params":[1,2,3]},{"id":1,"method":"test_method","params":[1,2,3]}]"#;
        let expected = Request::Batch(vec![
            Payload::Notification(Notification {
                method: "test_method".to_owned(),
                params: vec![1, 2, 3],
            }),
            Payload::Call(Call {
                id: 1,
                method: "test_method".to_owned(),
                params: vec![1, 2, 3],
            }),
        ]);

        let result: Request<M, P> = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn request_serialization() {
        // Single Notification.
        let input: Request<M, P> = Request::Single(Payload::Notification(Notification {
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        }));
        let expected = r#"{"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Single Call.
        let input = Request::Single(Payload::Call(Call {
            id: 1,
            method: "test_method".to_owned(),
            params: vec![1, 2, 3],
        }));
        let expected = r#"{"id":1,"method":"test_method","params":[1,2,3]}"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);

        // Batch Notification and Call.
        let input = Request::Batch(vec![
            Payload::Notification(Notification {
                method: "test_method".to_owned(),
                params: vec![1, 2, 3],
            }),
            Payload::Call(Call {
                id: 1,
                method: "test_method".to_owned(),
                params: vec![1, 2, 3],
            }),
        ]);
        let expected =
            r#"[{"method":"test_method","params":[1,2,3]},{"id":1,"method":"test_method","params":[1,2,3]}]"#;

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(result, expected);
    }
}
