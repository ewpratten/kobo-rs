# kobo-rs

[![Crates.io](https://img.shields.io/crates/v/kobo)](https://crates.io/crates/kobo) 
[![Docs.rs](https://docs.rs/kobo/badge.svg)](https://docs.rs/kobo) 
[![Build](https://github.com/Ewpratten/kobo-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/kobo-rs/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/kobo-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/kobo-rs/actions/workflows/clippy.yml)


`kobo-rs` is a minimal Rust library for interacting with modified Kobo e-readers. This is designed for use in applications running *on* the Kobo, not over the network.

## Building

Since we are cross-compiling (please don't try compiling software *on* a Kobo), we need a few tools.

```sh
docker pull ewpratten/kobo-cross-armhf:latest
cargo install cross
```

With that out of the way, the library can be built with:

```sh
cross build --target arm-unknown-linux-musleabihf
```

You'll need to do these same steps with your own applications.

## Running an example

To build and run the `display` example, you'll need to do the following:

```sh
cross build --target arm-unknown-linux-musleabihf --release --example display
```

Then, copy the binary to the Kobo's internal storage and execute the following command on the device:

```sh
/mnt/onboard/display
```

