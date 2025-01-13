use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SchemaSniffer {
    value_counts: HashMap<String, HashMap<Value, usize>>,
}

impl SchemaSniffer {
    /// Determines if an object should be treated as having dynamic properties
    /// based on its observed values
    fn is_dynamic_object(values: &HashMap<Value, usize>) -> bool {
        let mut key_consistency = HashMap::new();
        let total_objects = values.values().sum::<usize>();
        
        // Count how many times each key appears
        for (value, count) in values {
            if let Value::Object(obj) = value {
                for key in obj.keys() {
                    *key_consistency.entry(key.clone()).or_insert(0) += count;
                }
            }
        }
        
        // Calculate what percentage of objects contain each key
        let key_coverage: Vec<f64> = key_consistency.values()
            .map(|&c| c as f64 / total_objects as f64)
            .collect();
        
        // If any keys appear in less than 100% of objects, treat as dynamic
        key_coverage.iter().any(|&coverage| coverage < 1.0)
    }

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
                if !arr.is_empty() {
                    // Use [] to indicate array items rather than specific indices
                    let new_path = format!("{}[]", path);
                    for val in arr {
                        self.accumulate_counts(&new_path, val);
                    }
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
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/my-schema",
            "type": "object",
            "$defs": {},
            "properties": {}
        });
        
        let mut enum_defs = HashMap::new();
        
        for (path, counts) in &self.value_counts {
            let mut current = &mut schema;
            let parts: Vec<&str> = path.split('.').collect();
            
            for part in parts {
                if !current["properties"].is_object() {
                    current["properties"] = json!({});
                }
                current = &mut current["properties"];
                
                // Handle array notation
                if part.ends_with("[]") {
                    let base_part = part.trim_end_matches("[]");
                    if !current[base_part].is_object() {
                        current[base_part] = json!({
                            "type": "array",
                            "items": {
                                "type": "object"
                            }
                        });
                    }
                    current = &mut current[base_part]["items"];
                } else {
                    if !current[part].is_object() {
                        current[part] = json!({
                            "type": "object"
                        });
                    }
                    current = &mut current[part];
                }
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
            
            // Handle object key patterns
            if types.contains(&"object") {
                // Track unique keys seen in objects at this path
                let mut key_counts = HashMap::new();
                
                for (value, count) in counts {
                    if let Value::Object(obj) = value {
                        for key in obj.keys() {
                            *key_counts.entry(key.clone()).or_insert(0) += count;
                        }
                    }
                }
                
                let total_keys: usize = key_counts.values().sum();
                let unique_keys = key_counts.len();
                
                println!("\nField: {}", path);
                println!("Total object keys: {}", total_keys);
                println!("Unique object keys: {}", unique_keys);
                println!("Key ratio: {}", total_keys as f64 / unique_keys as f64);
                
                let is_dynamic = Self::is_dynamic_object(counts);
                
                if is_dynamic {
                    // For dynamic objects, clear any detected properties and set additionalProperties
                    if let Some(obj) = current.as_object_mut() {
                        obj.remove("properties");
                        obj.insert("additionalProperties".to_string(), json!(true));
                    }
                }
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
                    
                    // Create a unique enum name based on the path
                    let enum_name = format!("{}Enum", path.replace('.', "_").replace('[', "").replace(']', ""));
                    println!("Creating enum {} with values: {:?}", enum_name, enum_values);
                    
                    // Store the enum definition
                    enum_defs.insert(enum_name.clone(), json!({
                        "type": "string",
                        "enum": enum_values
                    }));
                    
                    // Use $ref instead of inline enum
                    current["$ref"] = json!(format!("#/$defs/{}", enum_name));
                }
            }
        }
        
        // Add all enum definitions to $defs
        if !enum_defs.is_empty() {
            schema["$defs"] = json!(enum_defs);
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
