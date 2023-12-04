# Test Case for https://github.com/bytecodealliance/componentize-py/issues/47

## Prerequisites

- `Rust`
- `componentize-py` 0.7.1

```
componentize-py -d wit -w hook componentize app -o process.wasm
cargo run --release
```
