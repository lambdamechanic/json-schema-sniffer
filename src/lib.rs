use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SchemaSniffer {
    value_counts: HashMap<String, HashMap<Value, usize>>,
}

impl SchemaSniffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_value(&mut self, value: &Value) {
        self.accumulate_counts("", value);
    }

    fn accumulate_counts(&mut self, path: &str, value: &Value) {
        match value {
            Value::Object(obj) => {
                for (key, val) in obj {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    self.accumulate_counts(&new_path, val);
                }
            }
            Value::Array(arr) => {
                for (i, val) in arr.iter().enumerate() {
                    let new_path = format!("{}[{}]", path, i);
                    self.accumulate_counts(&new_path, val);
                }
            }
            _ => {
                let counts = self.value_counts.entry(path.to_string())
                    .or_insert_with(HashMap::new);
                *counts.entry(value.clone()).or_insert(0) += 1;
            }
        }
    }

    pub fn infer_schema(&self) -> Value {
        let mut schema = json!({});
        
        for (path, counts) in &self.value_counts {
            let mut current = &mut schema;
            let parts: Vec<&str> = path.split('.').collect();
            
            for part in parts {
                if !current["properties"].is_object() {
                    current["properties"] = json!({});
                }
                current = &mut current["properties"];
                
                if !current[part].is_object() {
                    current[part] = json!({});
                }
                current = &mut current[part];
            }
            
            // Calculate total occurrences for this path
            let total_values: usize = counts.values().sum();
            
            // Determine the type(s) for this field
            let mut types = counts.keys()
                .map(|v| match v {
                    Value::Null => "null",
                    Value::Bool(_) => "boolean",
                    Value::Number(n) => {
                        if n.is_i64() || n.is_u64() { "integer" } else { "number" }
                    },
                    Value::String(_) => "string",
                    _ => "object",
                })
                .collect::<std::collections::HashSet<&str>>()
                .into_iter()
                .collect::<Vec<&str>>();
            
            // Sort types for consistent output
            types.sort();
            
            // JSON Schema requires type to be either a string or array of strings
            if types.len() == 1 {
                current["type"] = json!(types[0]);
            } else if types.len() > 1 {
                current["type"] = json!(types);
            }
            
            // Handle enum cases only for strings with limited unique values
            if types.contains(&"string") {
                let unique_strings = counts.keys()
                    .filter(|v| v.is_string())
                    .count();
                
                // Only create enum if:
                // 1. Less than 100 unique strings AND
                // 2. At least 10x as many entries as unique strings
                println!("\nField: {}", path);
                println!("Total values: {}", total_values);
                println!("Unique strings: {}", unique_strings);
                println!("10x rule: {}", total_values >= unique_strings * 10);
                
                if unique_strings < 100 && total_values >= unique_strings * 10 {
                    let enum_values: Vec<&Value> = counts.keys()
                        .filter(|v| v.is_string())
                        .collect();
                    println!("Creating enum with values: {:?}", enum_values);
                    current["enum"] = json!(enum_values);
                }
            }
        }
        
        schema
    }
}

pub fn validate_with_inferred_schema(values: Vec<Value>) -> Result<(jsonschema::Validator, Value), jsonschema::ValidationError<'static>> {
    let mut sniffer = SchemaSniffer::new();
    for value in values {
        sniffer.add_value(&value);
    }
    let schema = sniffer.infer_schema();
    Ok((jsonschema::validator_for(&schema)?, schema))
}
