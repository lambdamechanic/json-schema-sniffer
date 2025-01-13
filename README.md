# JSON Schema Sniffer

A Rust library for automatically inferring JSON Schemas from example JSON data.

## Features

- Infers JSON Schema Draft 2020-12 compliant schemas
- Handles nested objects and arrays
- Detects common patterns like enums
- Supports dynamic object properties
- Generates reusable schema definitions

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
json-schema-sniffer = "0.1"
```

## Usage

### Basic Example

```rust
use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

let values = vec![
    json!({"name": "Alice", "age": 30, "active": true}),
    json!({"name": "Bob", "age": 25, "active": false}),
];

let (validator, _schema) = validate_with_inferred_schema(values).unwrap();

// Validate new data
let test_value = json!({"name": "Charlie", "age": 35, "active": true});
assert!(validator.is_valid(&test_value));
```

### Handling Enums

The sniffer automatically detects string enums:

```rust
use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

let values = vec![
    json!({"status": "active"}),
    json!({"status": "inactive"}),
    json!({"status": "pending"}),
];

let (validator, _schema) = validate_with_inferred_schema(values).unwrap();

// Will fail validation
let invalid = json!({"status": "unknown"});
assert!(!validator.is_valid(&invalid));
```

### Nested Structures

Handles complex nested objects and arrays:

```rust
use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

let values = vec![
    json!({
        "name": "Alice",
        "address": {
            "street": "123 Main St",
            "city": "Springfield"
        },
        "hobbies": ["reading", "swimming"]
    }),
    json!({
        "name": "Bob",
        "address": {
            "street": "456 Elm St",
            "city": "Shelbyville"
        },
        "hobbies": ["gaming"]
    })
];

let (validator, _schema) = validate_with_inferred_schema(values).unwrap();
```

### Dynamic Properties

Supports objects with varying properties:

```rust
use json_schema_sniffer::validate_with_inferred_schema;
use serde_json::json;

let values = vec![
    json!({
        "type": "user",
        "name": "Alice",
        "email": "alice@example.com"
    }),
    json!({
        "type": "admin",
        "name": "Bob",
        "permissions": ["read", "write"]
    })
];

let (validator, _schema) = validate_with_inferred_schema(values).unwrap();
```

## API

### SchemaSniffer

- `new()` - Create a new schema sniffer
- `add_value(&mut self, value: &Value)` - Add a JSON value to analyze
- `infer_schema(&self) -> Value` - Generate JSON Schema from analyzed values

### validate_with_inferred_schema

- `validate_with_inferred_schema(values: Vec<Value>) -> Result<(Validator, Value)>`
  - Takes a vector of JSON values
  - Returns a validator and inferred schema

## Testing

Run tests with:

```bash
cargo test
```

## License

MIT
