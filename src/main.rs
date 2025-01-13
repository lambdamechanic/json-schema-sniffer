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
        // Adding more entries to meet 10x rule
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
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
            "eye-colour": "brown"
        }),
        json!({
            "name": "Zack",
            "age": 31,
            "active": false,
            "eye-colour": "blue"
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
    
    // Test with valid eye color
    let test_value = json!({
        "name": "Charlie",
        "age": 35,
        "active": true,
        "eye-colour": "blue"
    });

    println!("\nTesting valid eye color (blue):");

    if validator.is_valid(&test_value) {
        println!("Test value matches inferred schema!");
    } else {
        println!("Test value does NOT match inferred schema:");
        for error in validator.iter_errors(&test_value) {
            println!("- {}", error);
        }
    }

    // Test with invalid eye color
    let invalid_test_value = json!({
        "name": "Charlie",
        "age": 35,
        "active": true,
        "eye-colour": "flashing"
    });

    println!("\nTesting invalid eye color (flashing):");
    if validator.is_valid(&invalid_test_value) {
        println!("Test value matches inferred schema!");
    } else {
        println!("Test value does NOT match inferred schema:");
        for error in validator.iter_errors(&invalid_test_value) {
            println!("- {}", error);
        }
    }
}
