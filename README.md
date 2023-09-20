# {{project-name}}

[![Test Status](https://github.com/zduny/rust-webapp-template/actions/workflows/rust.yml/badge.svg)](https://github.com/zduny/rust-webapp-template/actions)

Rust web application template.

Implements a simple chat application.

Includes:
- [warp](https://github.com/seanmonstar/warp) server,
- browser client,
- [Web Worker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers) for client,
- native client,
- `common` library project for sharing code between the above.


## Prerequisites

To build the app [wasm-pack](https://rustwasm.github.io/wasm-pack) needs to be installed.

## How to use

Use [cargo-generate](https://github.com/cargo-generate/cargo-generate):

```bash
cargo generate --git https://github.com/zduny/rust-webapp-template
```

## To template users

Remember to update `README.md`, `LICENSE` and `Cargo.toml` files after creating new project using this template. 

## Example

Slightly simpler and more colorful example is available [here](https://github.com/zduny/balls/).

## Development

Use shell scripts to format code, lint, build, test, run or clean:

```bash
./format.sh
./clippy.sh
./build.sh
./test.sh
./run.sh
./build_and_run.sh
./clean.sh
```

Native client isn't included in `run.sh` (and `build_and_run.sh`) script,
to run native client (most likely in another terminal window/tab) type:

```bash
./app_client
```

### Windows 

Above scripts are available in Batch file form in `windows` directory.

**NOTE**: They have to be run from said `windows` directory, don't move them to project root directory before running them.

## Dependencies

This template uses following Rust crates developed by [me](https://github.com/zduny):

- [js-utils](https://github.com/zduny/js-utils) - various JavaScript related utilities.

- [mezzenger](https://github.com/zduny/mezzenger) - message passing infrastructure for Rust.

- [kodec](https://github.com/zduny/kodec) - message encoding/decoding interface.

Please consider donating to support their further development:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/O5O31JYZ4)

## See also

[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

[web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)

[js_sys](https://docs.rs/js-sys/latest/js_sys/)
