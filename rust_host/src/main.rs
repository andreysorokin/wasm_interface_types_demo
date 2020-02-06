//use wasmtime::{Config, Engine, Module, Store, Strategy};

#[wasmtime_rust::wasmtime]
trait WasmJsonModule {
    fn echo(&mut self, input: &str) -> String;
    fn json_top_keys(&mut self, input: String) -> String;
}

fn main() -> Result<(), failure::Error> {

    let input = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Same stuff is going on here, thread 'main' panicked at 'called `Result::unwrap()`
    // on an `Err` value: tables count must be at most 1 (at offset 434)', src/libcore/result.rs:1165:5
/*    let engine = wasmtime::Engine::new(&wasmtime::Config::new().wasm_multi_value(true));
    let store = wasmtime::Store::new(&engine);

    let bytes: Vec<u8> =  std::fs::read("rust_wt_js_demo.wasm")?;
    let module = Module::new(&store, bytes.as_ref()).unwrap();*/


    let mut wasm_json_module = WasmJsonModule::load_file("rust_wt_js_demo.wasm").unwrap(); // vs ?
//    let mut wasm_json_module = WasmJsonModule::load_file("rust_wt_js_demo.wasi.wasm").unwrap(); // vs ?
    println!("{}", wasm_json_module.json_top_keys(input.to_string()));
    println!("{}", wasm_json_module.echo("OMG_ECHO"));
    Ok(())
}
