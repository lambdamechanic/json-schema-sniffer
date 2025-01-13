use json_schema_sniffer::SchemaSniffer;
use serde_json::{json, Value};
use std::collections::HashMap;

#[test]
fn test_static_object_detection() {
    let mut values = HashMap::new();
    values.insert(
        json!({
            "strength": { "base": 10, "modifier": 0 },
            "dexterity": { "base": 12, "modifier": 1 }
        }),
        1
    );
    values.insert(
        json!({
            "strength": { "base": 15, "modifier": 2 },
            "dexterity": { "base": 14, "modifier": 2 }
        }),
        1
    );

    assert!(!SchemaSniffer::is_dynamic_object(&values),
        "Object with consistent keys should not be dynamic");
}

#[test]
fn test_dynamic_object_detection() {
    let mut values = HashMap::new();
    values.insert(
        json!({
            "favorite_color": { "value": "blue", "since": 2020 }
        }),
        1
    );
    values.insert(
        json!({
            "preferred_language": { "value": "Rust", "proficiency": "expert" }
        }),
        1
    );

    assert!(SchemaSniffer::is_dynamic_object(&values),
        "Object with different keys should be dynamic");
}

#[test]
fn test_mixed_keys_object_detection() {
    let mut values = HashMap::new();
    values.insert(
        json!({
            "common": "value1",
            "only_first": "unique"
        }),
        1
    );
    values.insert(
        json!({
            "common": "value2",
            "only_second": "unique"
        }),
        1
    );

    assert!(SchemaSniffer::is_dynamic_object(&values),
        "Object with mixed keys should be dynamic");
}
