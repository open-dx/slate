# Web Slate

Experimental support for Slate on the Web.

Reference: https://rustwasm.github.io/docs/book/

## Build and Run

Run both `wasm-pack ..` and `cargo [build|run] ..` for every change:

```sh
wasm-pack build --release --target web --out-name slate && cargo run --example basic
```

## Notes:

-   Build process is a bit gross atm. Because wasm-pack also needs to take control of the build directory for the current module, which means we can't use a build script to generate wasm files.
    -   TODO: Explore ways to fix this!
