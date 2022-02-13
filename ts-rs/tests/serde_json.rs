#![cfg(feature = "serde-json-impl")]
#![allow(unused)]

use ts_rs::TS;
use serde_json::Value;

#[test]
fn contains_serde_json() {
    #[derive(TS)]
    struct ContainsJson {
        field: Value,
    }

    assert_eq!(ContainsJson::decl(), "interface ContainsJson { field: any, }")
}


#[test]
fn newtype_serde_json() {
    #[derive(TS)]
    struct NewTypeJson(Value);    

    assert_eq!(NewTypeJson::decl(), "type NewTypeJson = any;");
}
