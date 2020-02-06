import { json_top_keys, echo } from './rust_wt_js_demo.wasm';

const data = `
{
    "name": "John Doe",
    "age": 43,
    "phones": [
    "+44 1234567",
    "+44 2345678"
     ]
}`;

console.log(echo("Echo goes here"));
console.log(json_top_keys(data));
