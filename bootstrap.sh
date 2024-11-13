#!/bin/sh

build () {
  wasm-pack build --target web
}

serve () {
  xdg-open http://localhost:8000 &
  python3 -m http.server
}

