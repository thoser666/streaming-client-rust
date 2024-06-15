use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};
use std::collections::VecDeque;
use std::vec::Vec;

/// Extension methods for the `Value` class, specifically for arrays.
pub trait JArrayExtensions {
    /// Converts a `Value` (JSON array) to a typed list of objects.
    fn to_typed_array<T>(&self) -> Result<Vec<T>, serde_json::Error>
    where
        T: DeserializeOwned;
}

impl JArrayExtensions for Value {
    fn to_typed_array<T>(&self) -> Result<Vec<T>, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        let mut results = Vec::new();
        if let Some(array) = self.as_array() {
            for item in array {
                results.push(from_value(item.clone())?);
            }
        }
        Ok(results)
    }
}

// Usage example (uncomment for actual use):
//
// use serde_json::json;
//
// #[derive(Debug, Deserialize)]
// struct MyStruct {
//     field: String,
// }
//
// fn main() {
//     let json_array = json!([
//         { "field": "value1" },
//         { "field": "value2" }
//     ]);
//
//     let result: Vec<MyStruct> = json_array.to_typed_array().unwrap();
//     for item in result {
//         println!("{:?}", item);
//     }
// }
