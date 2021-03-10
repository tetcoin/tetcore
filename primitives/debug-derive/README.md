## debug-derive

Macros to derive runtime debug implementation.

This custom derive implements a `core::fmt::Debug` trait,
but in case the `std` feature is enabled the implementation
will actually print out the structure as regular `derive(Debug)`
would do. If `std` is disabled the implementation will be empty.

This behaviour is useful to prevent bloating the runtime WASM
blob from unneeded code.

```rust
#[derive(debug_derive::RuntimeDebug)]
struct MyStruct;

assert_eq!(format!("{:?}", MyStruct), "MyStruct");
```
