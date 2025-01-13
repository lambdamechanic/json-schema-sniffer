use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

fn get_basic_test_values() -> Vec<serde_json::Value> {
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

fn get_dnd_test_values() -> Vec<serde_json::Value> {
    vec![
        json!({
            "name": "Gandalf",
            "class": "Wizard",
            "level": 20,
            "stats": {
                "strength": 10,
                "dexterity": 12,
                "constitution": 14,
                "intelligence": 20,
                "wisdom": 18,
                "charisma": 16
            },
            "inventory": {
                "weapons": [
                    {"name": "Staff", "damage": "1d6", "type": "bludgeoning"},
                    {"name": "Glamdring", "damage": "2d6", "type": "slashing"}
                ],
                "armor": [
                    {"name": "Robe", "ac": 10},
                    {"name": "Cloak of Protection", "ac": 12}
                ],
                "misc": {
                    "potions": 3,
                    "scrolls": 5,
                    "gold": 150
                }
            },
            "spells": [
                {"name": "Fireball", "level": 3, "school": "Evocation"},
                {"name": "Mage Armor", "level": 1, "school": "Abjuration"}
            ]
        }),
        json!({
            "name": "Aragorn",
            "class": "Ranger",
            "level": 15,
            "stats": {
                "strength": 18,
                "dexterity": 16,
                "constitution": 16,
                "intelligence": 14,
                "wisdom": 14,
                "charisma": 12
            },
            "inventory": {
                "weapons": [
                    {"name": "Andúril", "damage": "2d6", "type": "slashing"}
                ],
                "armor": [
                    {"name": "Chainmail", "ac": 16}
                ],
                "misc": {
                    "potions": 1,
                    "scrolls": 0,
                    "gold": 50
                }
            },
            "spells": []
        })
    ]
}

#[test]
fn test_schema_inference() {
    let values = get_basic_test_values();

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
fn test_deeply_nested_structure() {
    let values = get_dnd_test_values();
    let (_, schema) = validate_with_inferred_schema(values)
        .expect("Failed to create validator");

    // Print the inferred schema
    println!("Inferred D&D schema:\n{}", serde_json::to_string_pretty(&schema).unwrap());

    // Check nested stats structure
    assert_eq!(schema["properties"]["stats"]["type"], "object");
    assert_eq!(schema["properties"]["stats"]["properties"]["strength"]["type"], "object");
    assert_eq!(schema["properties"]["stats"]["properties"]["dexterity"]["type"], "object");
    
    // Check the nested structure
    assert_eq!(schema["properties"]["stats"]["properties"]["strength"]["properties"]["base"]["type"], "integer");
    assert_eq!(schema["properties"]["stats"]["properties"]["strength"]["properties"]["modifier"]["type"], "integer");

    // Check nested inventory structure
    assert_eq!(schema["properties"]["inventory"]["type"], "object");
    assert_eq!(schema["properties"]["inventory"]["properties"]["weapons"]["type"], "array");
    assert_eq!(schema["properties"]["inventory"]["properties"]["weapons"]["items"]["type"], "object");
    assert_eq!(schema["properties"]["inventory"]["properties"]["weapons"]["items"]["properties"]["name"]["type"], "string");
    assert_eq!(schema["properties"]["inventory"]["properties"]["weapons"]["items"]["properties"]["damage"]["type"], "string");

    // Check deeply nested misc items
    assert_eq!(schema["properties"]["inventory"]["properties"]["misc"]["type"], "object");
    assert_eq!(schema["properties"]["inventory"]["properties"]["misc"]["properties"]["potions"]["type"], "integer");
    assert_eq!(schema["properties"]["inventory"]["properties"]["misc"]["properties"]["gold"]["type"], "integer");

    // Check spells array structure
    assert_eq!(schema["properties"]["spells"]["type"], "array");
    assert_eq!(schema["properties"]["spells"]["items"]["type"], "object");
    assert_eq!(schema["properties"]["spells"]["items"]["properties"]["name"]["type"], "string");
    assert_eq!(schema["properties"]["spells"]["items"]["properties"]["level"]["type"], "integer");
}

#[test]
fn test_gimli_validation() {
    let values = get_dnd_test_values();
    let (validator, _) = validate_with_inferred_schema(values)
        .expect("Failed to create validator");

    // Create Gimli's character data
    let gimli = json!({
        "name": "Gimli",
        "class": "Fighter",
        "level": 12,
        "stats": {
            "strength": 18,
            "dexterity": 10,
            "constitution": 16,
            "intelligence": 10,
            "wisdom": 12,
            "charisma": 8
        },
        "inventory": {
            "weapons": [
                {"name": "Battle Axe", "damage": "1d10", "type": "slashing"}
            ],
            "armor": [
                {"name": "Chainmail", "ac": 16}
            ],
            "misc": {
                "potions": 2,
                "scrolls": 0,
                "gold": 75
            }
        },
        "spells": []
    });

    // Validate Gimli against the schema
    assert!(validator.is_valid(&gimli), "Gimli's character data should validate against the schema");

    // Test invalid data
    let invalid_gimli = json!({
        "name": "Gimli",
        "class": "Fighter",
        "level": "twelve", // Invalid: level should be integer
        "stats": {
            "strength": 18,
            "dexterity": 10,
            "constitution": 16,
            "intelligence": 10,
            "wisdom": 12,
            "charisma": 8
        },
        "inventory": {
            "weapons": [
                {"name": "Battle Axe", "damage": "1d10", "type": "slashing"}
            ],
            "armor": [
                {"name": "Chainmail", "ac": 16}
            ],
            "misc": {
                "potions": 2,
                "scrolls": 0,
                "gold": 75
            }
        },
        "spells": []
    });

    assert!(!validator.is_valid(&invalid_gimli), "Invalid level type should fail validation");
}

#[test]
fn test_dynamic_vs_static_objects() {
    let values = vec![
        // Static object example with object values
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
        }),
        // Dynamic object example
        json!({
            "metadata": {
                "favorite_color": { "value": "blue", "since": 2020 },
                "age": { "value": 30, "unit": "years" }
            }
        }),
        json!({
            "metadata": {
                "preferred_language": { "value": "Rust", "proficiency": "expert" },
                "experience_years": { "value": 5, "details": "professional" }
            }
        }),
        json!({
            "metadata": {
                "hobby": { "value": "programming", "frequency": "daily" },
                "coffee_cups_per_day": { "value": 3, "preferred_type": "espresso" }
            }
        })
    ];

    let (validator, schema) = validate_with_inferred_schema(values)
        .expect("Failed to create validator");

    // Print the inferred schema
    println!("Inferred schema:\n{}", serde_json::to_string_pretty(&schema).unwrap());

    // Test static object behavior
    assert_eq!(schema["properties"]["stats"]["type"], "object");
    assert_eq!(schema["properties"]["stats"]["properties"]["strength"]["type"], "object");
    assert_eq!(schema["properties"]["stats"]["properties"]["dexterity"]["type"], "object");
    assert!(!schema["properties"]["stats"].get("additionalProperties").is_some(), 
        "Static object should not have additionalProperties");

    // Test dynamic object behavior
    assert_eq!(schema["properties"]["metadata"]["type"], "object");
    assert!(schema["properties"]["metadata"].get("additionalProperties").is_some(), 
        "Dynamic object should have additionalProperties");
    assert_eq!(schema["properties"]["metadata"]["additionalProperties"], true);
    assert!(schema["properties"]["metadata"]["properties"].as_object().unwrap().is_empty(),
        "Dynamic object should have empty properties");
}

#[test]
fn test_schema_structure() {
    let values = get_basic_test_values();

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
