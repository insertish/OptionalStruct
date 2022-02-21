#[macro_use]
extern crate optional_struct;

use serde::Serialize;

#[derive(OptionalStruct, Serialize)]
#[optional_derive(Serialize)]
#[opt_skip_serializing_none]
struct Data {
    test: Option<String>,
}

#[test]
fn test_serde() {
    let data = OptionalData {
        test: Some("value".into()),
    };

    assert_eq!(serde_json::to_string(&data).unwrap(), "{\"test\":\"value\"}");
    
    let data = OptionalData {
        test: None,
    };

    assert_eq!(serde_json::to_string(&data).unwrap(), "{}");
}