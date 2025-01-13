use json_schema_sniffer::{validate_with_inferred_schema, SchemaSniffer};
use serde_json::json;

fn main() {
    let values = vec![
        json!({
            "name": "Alice",
            "age": 30,
            "active": true
        }),
        json!({
            "name": "Bob",
            "age": 25,
            "active": false
        }),
    ];

    let validator = match validate_with_inferred_schema(values) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to create validator: {}", e);
            std::process::exit(1);
        }
    };
    
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
