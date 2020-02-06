# Prerequisites

   Build rust_wt_js_demo.wasm and copy it to top-level directory, see README.md in rust_wt_js_demo directory for details.
   don't forget to clean lock files and modules when switching to another node version


# Running node js

Tested with node v13.0.0 - 13.6.0, 13.7.0 may fail due to the recent changes

    node --experimental-wasm-mv --experimental-modules --loader ./loader.mjs ./run.mjs