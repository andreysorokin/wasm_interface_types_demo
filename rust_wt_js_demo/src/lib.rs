use serde_json::{json, Value};
use linked_hash_map::LinkedHashMap as Map;
use wasm_bindgen::prelude::*;

//func json_top_keys (param string) (result string)

#[wasm_bindgen]
pub fn echo(input: &str) -> String {
    return input.to_string();
}

#[wasm_bindgen]
pub fn json_top_keys(input: String) -> String {
    let input_json: Map<String, Value> = serde_json::from_str(&input).unwrap();
    let top_keys: Vec<String> = input_json.into_iter().map(|(k,_v)| k).collect();
    let response = json!({
      "topKeys": top_keys
    });
    return serde_json::to_string(&response).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::json_top_keys;

    #[test]
    fn check_json_top_response() {
        let input = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let output = r#"{"topKeys":["name","age","phones"]}"#;
        assert_eq!(output.to_string(), json_top_keys(input.into()));
    }
}
