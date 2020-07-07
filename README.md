# cpp_with_rust_call_web_api
Example project on how to use rust to call a JSON web API from C++

# Requirements
- Make
- [rust](https://rustup.rs/)
- [cbindgen](https://github.com/eqrion/cbindgen/#quick-start)

# How to build
`make`

# Points of interest
An example object is provided in [rust/src/objects/graph.rs](https://github.com/VictorKoenders/cpp_with_rust_call_web_api/blob/master/rust/src/objects/graph.rs)

This is called from [main.cpp](https://github.com/VictorKoenders/cpp_with_rust_call_web_api/blob/master/main.cpp#L29)

The `rust_web_api_caller.h` is automatically generated based on Rust code.

This works on all [supported platforms!](https://forge.rust-lang.org/release/platform-support.html)
