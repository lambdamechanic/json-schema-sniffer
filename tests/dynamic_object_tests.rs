use json_schema_sniffer::SchemaSniffer;
use serde_json::json;

#[test]
fn test_static_object_detection() {
    let values = vec![
        json!({
            "stats": {
                "strength": { "base": 10, "modifier": 0 },
                "dexterity": { "base": 12, "modifier": 1 }
            }
        }),
        json!({
            "stats": {
                "strength": { "base": 15, "modifier": 2 },
                "dexterity": { "base": 14, "modifier": 2 }
            }
        })
    ];

    let mut sniffer = SchemaSniffer::new();
    for value in values {
        sniffer.add_value(&value);
    }
    
    let schema = sniffer.infer_schema();
    
    assert_eq!(schema["properties"]["stats"]["type"], "object");
    assert!(!schema["properties"]["stats"].get("additionalProperties").is_some(),
        "Static object should not have additionalProperties");
}

#[test]
fn test_dynamic_object_detection() {
    let values = vec![
        json!({
            "metadata": {
                "favorite_color": { "value": "blue", "since": 2020 }
            }
        }),
        json!({
            "metadata": {
                "preferred_language": { "value": "Rust", "proficiency": "expert" }
            }
        })
    ];

    let mut sniffer = SchemaSniffer::new();
    for value in values {
        sniffer.add_value(&value);
    }
    
    let schema = sniffer.infer_schema();
    
    assert_eq!(schema["properties"]["metadata"]["type"], "object");
    assert!(schema["properties"]["metadata"].get("additionalProperties").is_some(),
        "Dynamic object should have additionalProperties");
    assert_eq!(schema["properties"]["metadata"]["additionalProperties"], true);
}

#[test]
fn test_mixed_keys_object_detection() {
    let values = vec![
        json!({
            "metadata": {
                "common": "value1",
                "only_first": "unique"
            }
        }),
        json!({
            "metadata": {
                "common": "value2",
                "only_second": "unique"
            }
        })
    ];

    let mut sniffer = SchemaSniffer::new();
    for value in values {
        sniffer.add_value(&value);
    }
    
    let schema = sniffer.infer_schema();
    
    assert_eq!(schema["properties"]["metadata"]["type"], "object");
    assert!(schema["properties"]["metadata"].get("additionalProperties").is_some(),
        "Object with mixed keys should have additionalProperties");
    assert_eq!(schema["properties"]["metadata"]["additionalProperties"], true);
}
