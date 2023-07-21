# Yujin

Yujin is an experiment to create a front-end for the [Many
protocol](https://github.com/many-protocol) using only browser-native APIs, i.e. zero
dependencies or JavaScript bundlers.

We accomplish this task by having a Service Worker intercept all requests with the
`X-Many-Request` header and handling the encoding and decoding of CBOR within a Rust-based
WebAssembly module.

## Getting Started

You'll need wasm-pack to get started.

```
% cargo install wasm-pack
```

And then you can build the project.

```
% wasm-pack build --target web --dev
```

The `web` target will build the Wasm ready to import into a browser.
The `dev` flag will send any Rust panics to your browser's console.

Then serve the project directory locally (with `http-server` or something).

## Usage

Currently, the front-end sends a precomputed CBOR payload to the server URL in the HTML
form. Future versions will encode the form inputs into CBOR, encode the request, decode
the response, and format it for consumption by the front-end.
