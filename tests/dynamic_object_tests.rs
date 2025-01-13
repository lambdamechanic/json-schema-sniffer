use json_schema_sniffer::SchemaSniffer;
use serde_json::json;
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

    let (has_dynamic_keys, static_keys) = SchemaSniffer::analyze_object_properties(&values);
    assert!(!has_dynamic_keys, "Object with consistent keys should not be dynamic");
    assert_eq!(static_keys.len(), 2, "Should have exactly two static keys");
    assert!(static_keys.iter().any(|k| k == "strength"), "Should identify 'strength' as static key");
    assert!(static_keys.iter().any(|k| k == "dexterity"), "Should identify 'dexterity' as static key");
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

    let (has_dynamic_keys, static_keys) = SchemaSniffer::analyze_object_properties(&values);
    assert!(has_dynamic_keys, "Object with different keys should be dynamic");
    assert!(static_keys.is_empty(), "Should have no static keys in fully dynamic object");
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

    let (has_dynamic_keys, static_keys) = SchemaSniffer::analyze_object_properties(&values);
    assert!(has_dynamic_keys, "Object with mixed keys should be dynamic");
    assert!(static_keys.iter().any(|k| k == "common"), "Should identify 'common' as static key");
    assert!(!static_keys.iter().any(|k| k == "only_first"), "Should not identify 'only_first' as static key");
}
