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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
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
