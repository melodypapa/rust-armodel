# rust-armodel

Rust library to parse and generate AUTOSAR ARXML files.

The module structure follows the AUTOSAR specification (R23-11): every model
class lives in the Rust module that mirrors its AUTOSAR spec package path.

## Project status

Work in progress. The current milestone (P0 walking skeleton) proves the full
pipeline end to end: parse an ARXML file into a typed Rust model, write it
back, and re-parse it into an equal model.

See `docs/superpowers/specs/2026-10-01-rust-armodel-p0-walking-skeleton-design.md`
for the design, and `docs/code_guide.md` for the coding guidelines.

## How to compile
1. Run `cargo build` to build the library and binaries in debug mode
2. Run `cargo build --release` to build with optimizations

## How to run
1. Run `cargo run --bin arxml-dump -- -a <file.arxml>` to run the `arxml-dump` tool on an ARXML file
   (use `-h` to show the help)

## How to generate the documentation
1. Run `cargo doc --open` to generate and open the API documentation

## How to perform the testing
1. Run `cargo test` to verify the library

## How to publish to crates.io
1. Run `cargo test` to verify all the tests are passed
2. Run `cargo publish` to upload the crate to crates.io
