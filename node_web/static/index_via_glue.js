import * as wt from "./rust_wt_js_demo.js";

const setupWasm = async () => {
  // Instantiate wasm module
  await wt.default("./rust_wt_js_demo_bg.wasm");

  const input = document.getElementById('input');
  const output = document.getElementById('output');

  function rerender() {
    output.innerHTML = wt.json_top_keys(input.value);
  }

  input.addEventListener('keyup', rerender);
  rerender();
};
setupWasm();