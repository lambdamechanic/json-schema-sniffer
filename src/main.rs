use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

fn main() {
    let values = vec![
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
        }),
    ];

    let (validator, schema) = match validate_with_inferred_schema(values) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to create validator: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("Inferred schema:\n{}", serde_json::to_string_pretty(&schema).unwrap());
    
    let test_value = json!({
        "name": "Charlie",
        "age": 35,
        "active": true
    });

    if validator.is_valid(&test_value) {
        println!("Test value matches inferred schema!");
    } else {
        println!("Test value does NOT match inferred schema:");
        for error in validator.iter_errors(&test_value) {
            println!("- {}", error);
        }
    }
}
