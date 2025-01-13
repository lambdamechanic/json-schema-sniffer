use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SchemaSniffer {
    value_counts: HashMap<String, HashMap<Value, usize>>,
}

impl SchemaSniffer {
    /// Analyzes an object's properties to determine which are static (present in all objects)
    /// versus dynamic (varying between objects)
    pub fn analyze_object_properties(values: &HashMap<Value, usize>) -> (bool, Vec<String>) {
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
        
        // Find keys that appear in 100% of objects
        let static_keys: Vec<String> = key_consistency.iter()
            .filter(|(_, &count)| (count as f64 / total_objects as f64) >= 1.0)
            .map(|(key, _)| key.clone())
            .collect();
            
        // Object is dynamic if any keys appear in less than 100% of objects
        let has_dynamic_keys = key_consistency.values()
            .any(|&count| (count as f64 / total_objects as f64) < 1.0);
            
        (has_dynamic_keys, static_keys)
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

    fn infer_basic_schema() -> Value {
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/my-schema",
            "type": "object",
            "$defs": {},
            "properties": {}
        })
    }

    fn infer_field_type(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() { "integer" } else { "number" }
            },
            Value::String(_) => "string",
            _ => "object",
        }
    }

    fn handle_object_properties(&self, current: &mut Value, counts: &HashMap<Value, usize>, path: &str) {
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
        
        let (has_dynamic_keys, static_keys) = Self::analyze_object_properties(counts);
        
        if has_dynamic_keys {
            if let Some(obj) = current.as_object_mut() {
                if let Some(props) = obj.get_mut("properties").and_then(|v| v.as_object_mut()) {
                    props.retain(|key, _| static_keys.contains(key));
                }
                obj.insert("additionalProperties".to_string(), json!(true));
            }
        }
    }

    fn handle_string_enums(&self, current: &mut Value, counts: &HashMap<Value, usize>, path: &str, enum_defs: &mut HashMap<String, Value>) {
        let total_values: usize = counts.values().sum();
        let unique_strings = counts.keys()
            .filter(|v| v.is_string())
            .count();
        
        println!("\nField: {}", path);
        println!("Total values: {}", total_values);
        println!("Unique strings: {}", unique_strings);
        println!("10x rule: {}", total_values >= unique_strings * 10);
        
        if unique_strings < 100 && total_values >= unique_strings * 10 {
            let enum_values: Vec<&Value> = counts.keys()
                .filter(|v| v.is_string())
                .collect();
            
            let enum_name = format!("{}Enum", path.replace('.', "_").replace('[', "").replace(']', ""));
            println!("Creating enum {} with values: {:?}", enum_name, enum_values);
            
            enum_defs.insert(enum_name.clone(), json!({
                "type": "string",
                "enum": enum_values
            }));
            
            current["$ref"] = json!(format!("#/$defs/{}", enum_name));
        }
    }

    pub fn infer_schema(&self) -> Value {
        let mut schema = Self::infer_basic_schema();
        let mut enum_defs = HashMap::new();
        
        for (path, counts) in &self.value_counts {
            let mut current = &mut schema;
            let parts: Vec<&str> = path.split('.').collect();
            
            for part in parts {
                if !current["properties"].is_object() {
                    current["properties"] = json!({});
                }
                current = &mut current["properties"];
                
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
            
            let mut types: Vec<_> = counts.keys()
                .map(|v| Self::infer_field_type(v))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            types.sort();
            
            if types.len() == 1 {
                current["type"] = json!(types[0]);
            } else if types.len() > 1 {
                current["type"] = json!(types);
            }
            
            if types.contains(&"object") {
                self.handle_object_properties(current, counts, path);
            }
            
            if types.contains(&"string") {
                self.handle_string_enums(current, counts, path, &mut enum_defs);
            }
        }
        
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
