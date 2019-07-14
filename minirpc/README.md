# minirpc

MINI-RPC is a stateless, light-weight remote procedure call (RPC) protocol. Inspired by the [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification), this implementation written in Rust selected its essential parts in order to keep as minimal as possible the communication between processes.

## Examples

Syntax:

```
--> data sent to Server
<-- data sent to Client
```

### Basic Usage

```
--> {"id":1,"method":"sum","params":[1,2,3]}
<-- {"id":1,"result":6}
```

```rust
use minirpc::*;
use serde_json::*;

// --> Request.
let input = r#"{"id":1,"method":"sum","params":[1,2,3]}"#;
let expected = Request::Single(RequestPayload::Call(Call {
    id: Id::Number(1),
    method: Method::String("sum".to_owned()),
    params: Params::Array(vec![Value::from(1), Value::from(2), Value::from(3)]),
}));

let result: Request = serde_json::from_str(input).unwrap();
assert_eq!(result, expected);

// <-- Response.
let input = r#"{"id":1,"result":6}"#;
let expected = Response::Single(ResponsePayload::Success(Success {
    id: Id::Number(1),
    result: Value::Bool(true),
}));

let result: Response = serde_json::from_str(input).unwrap();
assert_eq!(result, expected);
```

### Batch Notification

```
--> [{"method":"log","params":{"level":"info"}},{"method":"log","params":{"level":"warn"}}]
```

```rust
// --> Batch Request.
let input = r#"[{"method":"log","params":{"level":"info"}},{"method":"log","params":{"level":"warn"}}]"#;

let mut params_1 = Map::new();
params_1.insert("level".to_string(), "info".into());

let mut params_2 = Map::new();
params_2.insert("level".to_string(), "warn".into());

let expected = Request::Batch(vec![
    RequestPayload::Notification(Notification {
        method: Method::String("log".to_owned()),
        params: Params::Object(params_1),
    }),
    RequestPayload::Notification(Notification {
        method: Method::String("log".to_owned()),
        params: Params::Object(params_2),
    }),
]);

let result: Request = serde_json::from_str(input).unwrap();
assert_eq!(result, expected);
```
