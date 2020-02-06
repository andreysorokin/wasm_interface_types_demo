import wasmtime
import rust_wt_js_demo

data = """
{
    "name": "John Doe",
    "age": 43,
    "phones": [
    "+44 1234567",
    "+44 2345678"
     ]
}"""
print(rust_wt_js_demo.json_top_keys(data))