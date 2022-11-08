# {{project-name}}

Rust web application template.

Implements a simple chat application.

Includes:
- [warp](https://github.com/seanmonstar/warp) server,
- browser client,
- [Web Worker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers) for client,
- native client,
- `common` library project for sharing code between the above.

## To template users

Remember to update `README.md`, `LICENSE` and `Cargo.toml` files after creating new project using this template. 

## Development

Use shell scripts to format code, lint, build, run or clean:

```bash
./format.sh
./clippy.sh
./build.sh
./run.sh
./build_and_run.sh
./clean.sh
```

## Dependencies

This template uses following Rust crates developed by [me](https://github.com/zduny)

[js-utils](https://github.com/zduny/js-utils) - various JavaScript related utilities.

[mezzenger](https://github.com/zduny/mezzenger) - message passing infrastructure for Rust.

[kodec](https://github.com/zduny/kodec) - message encoding/decoding interface.
