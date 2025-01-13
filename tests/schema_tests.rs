use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

fn get_test_values() -> Vec<serde_json::Value> {
    vec![
        json!({
            "name": "Alice",
            "age": 30,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Bob",
            "age": 25,
            "active": false,
            "eye-colour": "green"
        }),
        json!({
            "name": "Charlie",
            "age": 35,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Diana",
            "age": 28,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Eve",
            "age": 32,
            "active": false,
            "eye-colour": "green"
        }),
        json!({
            "name": "Frank",
            "age": 40,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Grace",
            "age": 22,
            "active": false,
            "eye-colour": "green"
        }),
        json!({
            "name": "Heidi",
            "age": 29,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Ivan",
            "age": 31,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Judy",
            "age": 27,
            "active": false,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Kevin",
            "age": 33,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Linda",
            "age": 26,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Michael",
            "age": 34,
            "active": false,
            "eye-colour": "green"
        }),
        json!({
            "name": "Nancy",
            "age": 29,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Oscar",
            "age": 30,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Paul",
            "age": 38,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Quinn",
            "age": 27,
            "active": false,
            "eye-colour": "green"
        }),
        json!({
            "name": "Rachel",
            "age": 31,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Steve",
            "age": 29,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Tina",
            "age": 33,
            "active": false,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Uma",
            "age": 28,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Victor",
            "age": 35,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Wendy",
            "age": 30,
            "active": false,
            "eye-colour": "green"
        }),
        json!({
            "name": "Xander",
            "age": 32,
            "active": true,
            "eye-colour": "blue"
        }),
        json!({
            "name": "Yvonne",
            "age": 29,
            "active": true,
            "eye-colour": "green"
        }),
        json!({
            "name": "Zack",
            "age": 31,
            "active": false,
            "eye-colour": "blue"
        }),
    ]
}

#[test]
fn test_schema_inference() {
    let values = get_test_values();

    let (validator, schema) = validate_with_inferred_schema(values)
        .expect("Failed to create validator");

    // Test with valid eye color
    let test_value = json!({
        "name": "Charlie",
        "age": 35,
        "active": true,
        "eye-colour": "blue"
    });

    assert!(validator.is_valid(&test_value), "Valid value should pass validation");

    // Test with invalid eye color
    let invalid_test_value = json!({
        "name": "Charlie",
        "age": 35,
        "active": true,
        "eye-colour": "flashing"
    });

    assert!(!validator.is_valid(&invalid_test_value), "Invalid value should fail validation");
    
    // Check that the error message contains the expected enum values
    let errors: Vec<String> = validator
        .iter_errors(&invalid_test_value)
        .map(|e| e.to_string())
        .collect();
    
    assert!(errors.iter().any(|e| e.contains("blue")), "Error should mention 'blue'");
    assert!(errors.iter().any(|e| e.contains("green")), "Error should mention 'green'");
}

#[test]
fn test_schema_structure() {
    let values = get_test_values();

    let (_, schema) = validate_with_inferred_schema(values)
        .expect("Failed to create validator");

    // Check basic schema structure
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"].is_object());
    
    // Check field types
    assert_eq!(schema["properties"]["name"]["type"], "string");
    assert_eq!(schema["properties"]["age"]["type"], "integer");
    assert_eq!(schema["properties"]["active"]["type"], "boolean");
    
    // Check enum reference
    assert!(schema["properties"]["eye-colour"]["$ref"].is_string());
    let ref_path = schema["properties"]["eye-colour"]["$ref"].as_str().unwrap();
    assert!(ref_path.starts_with("#/$defs/"));
    
    // Check enum definition
    let enum_name = ref_path.trim_start_matches("#/$defs/");
    assert!(schema["$defs"][enum_name].is_object());
    assert_eq!(schema["$defs"][enum_name]["type"], "string");
    assert!(schema["$defs"][enum_name]["enum"].is_array());
}
