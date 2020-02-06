# Rust provider notes

## Function definition

    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    pub fn json_top_keys(input: String) -> String
    
    
## Build options

     WASM_INTERFACE_TYPES=1 wasm-pack build --release
    
The release artifact will be available in `pkg` directory