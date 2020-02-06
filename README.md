# Wasm type JSON demo

The scenario is very simple. 
An input is a UTF-8 input string representing JSON object. 
Top-level keys are collected and returned in an output JSON as array.

Input:

    { 
      "key1": "Some value",
      "key2": [0, 2, 3, 5],
      "keyN": {"intKey1": "v1", "intLey2": {"mk1":[1,2]}}
    }
    
Output:
    
    {
      "topKeys": ["key1", "key2", "keyN"]
    }
    
# Pre-requisites

* The project is built with wasm-pack v0.8.1, force it with `cargo install wasm-pack --vers 0.8.1`
* Rust host and provider libraries are linked against exact `wasm-bindgen = "=0.2.55"` version

# Interface specification

    func json_top_keys (param string) (result string)
    
Where `string` contains well-formed JSON object.
    
# Target dev languages

- Rust [X]
- C    [ ]
- C++. [ ]


# Target platforms (hosts)
- [X] Web (JS+WASM) - works for Chrome with experimental flags, glue code required for other browsers 
- [!] Wasmtime - need to patch CLI to enable multivalve return 
- [X] Rust host - works just fine 
- [X!] Node - works for version 13.0.0 - 13.6.0.
- [ ] C/C++ host
- [ ] Python host
