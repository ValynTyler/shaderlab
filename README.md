# Hello, Rust WASM!
A small project following the [MDN docs intro to Rust WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm) guide.

To build the project for the web, use `wasm-pack build --target web`, then run it by opening this directory on a local server:
```bash
xdg-open http://localhost:8000 &
python3 -m http.server
```
## Resources
- https://docs.rs/web-sys/0.3.72/web\_sys/index.html
