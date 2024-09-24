# rust-flock-of-boids
## Description
A fork of camsjams/rust-flock-of-boids designed to replicate an existing swarm and target problem written in java.

## Version
See [Cargo.toml](Cargo.toml) version

## Platforms / Technologies
* [Rust](https://www.rust-lang.org/en-US/)
* [Piston](https://www.piston.rs/)

## Run
Note: see [Cargo.toml](Cargo.toml) to swap between Tetra/Bevy due to conflicts of native libraries
>      $ cargo run --bin with_piston

Another Note: I chose _not_ to use [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) due to the format of the video and to keep the code simple and concise.

## Build For Release
>      $ cargo build --bin with_piston --release