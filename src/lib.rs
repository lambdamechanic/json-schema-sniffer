use jsonschema::{JSONSchema, Draft};
use serde_json::Value;
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
        let mut schema = serde_json::json!({});
        
        for (path, counts) in &self.value_counts {
            let mut current = &mut schema;
            let parts: Vec<&str> = path.split('.').collect();
            
            for part in parts {
                current = current.entry("properties")
                    .or_insert_with(|| json!({}))
                    .as_object_mut()
                    .unwrap()
                    .entry(part)
                    .or_insert_with(|| json!({}));
            }
            
            let types: Vec<&str> = counts.keys()
                .map(|v| match v {
                    Value::Null => "null",
                    Value::Bool(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    _ => "object",
                })
                .collect();
            
            if types.len() == 1 {
                current["type"] = json!(types[0]);
            } else {
                current["type"] = json!(types);
            }
            
            if counts.len() <= 5 {
                let enum_values: Vec<&Value> = counts.keys().collect();
                current["enum"] = json!(enum_values);
            }
        }
        
        schema
    }
}

pub fn validate_with_inferred_schema(values: Vec<Value>) -> Result<JSONSchema, jsonschema::ValidationError> {
    let mut sniffer = SchemaSniffer::new();
    for value in values {
        sniffer.add_value(&value);
    }
    let schema = sniffer.infer_schema();
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
}
