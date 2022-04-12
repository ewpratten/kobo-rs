# kobo-rs
[![Crates.io](https://img.shields.io/crates/v/kobo)](https://crates.io/crates/kobo) 
[![Docs.rs](https://docs.rs/kobo/badge.svg)](https://docs.rs/kobo) 
[![Build](https://github.com/Ewpratten/kobo-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/kobo-rs/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/kobo-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/kobo-rs/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/kobo-rs/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/kobo-rs/actions/workflows/audit.yml)


repo description

## Building

Since we are cross-compiling (please don't try compiling software *on* a Kobo), we need a few tools.

```sh
docker pull ewpratten/kobo-cross-armhf:latest
cargo install cross
```

With that out of the way, the library can be built with:

```sh
cross build --target arm-unknown-linux-gnueabihf
```

You'll need to do these same steps with your own applications.
