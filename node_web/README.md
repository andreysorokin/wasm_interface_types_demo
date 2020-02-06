# Packaging for Web

## WASM via JS glue code (All browsers fallback)

In order to build wasm inside rust_wt_js_demo run:
    
     WASM_INTERFACE_TYPES=0 wasm-pack build --release --target web
     cp pkg/rust_wt_js_demo_bg.wasm ../node_web/static
     cp pkg/rust_wt_js_demo.js ../node_web/static
     
Run this project `npm start` and navigate to [http://localhost:8080/index_glue.html](http://localhost:8080/index_glue.html)


## Experimental WASM Modules (Chrome)

Chrome supports multivalue return from functions in experimental mode, which makes possible to use WASM interface types.

There's some boilerplate code in index_via_glue_mv.js file. Please, note the differences with memory management and multivalues returned.
This code can be generated with webpack, see [https://github.com/bytecodealliance/wasmtime-demos/tree/master/webpack](https://github.com/bytecodealliance/wasmtime-demos/tree/master/webpack) for details.

In order to build wasm, follow the generic instructions, inside rust_wt_js_demo run:

     WASM_INTERFACE_TYPES=1 wasm-pack build --release
     cp pkg/rust_wt_js_demo.wasm ../node_web/static
     
Then go to [chrome://flags/](chrome://flags/) and turn on `Experimental WebAssembly` flag.

Run this project  `npm start`  and navigate to [http://localhost:8080/index_glue_mv.html](http://localhost:8080/index_glue_mv.html)
