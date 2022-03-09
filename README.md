# Samos

Operating system time!

Guide via <https://os.phil-opp.com/>

## Somethings that might be needed

```sh
rustup component add rust-src
rustup target add thumbv7em-none-eabihf # Build to an embedded target.
cargo build --target thumbv7em-none-eabihf
cargo rustc -- -C link-arg=-nostartfiles

rustup toolchain install nightly
rustup toolchain install list
rustup component add rust-src --toolchain nightly
rustup component add rust-src --toolchain nightly x86_64-samos
rustup override set nightly

cargo build --target x86_64-samos.json
```

## Done

<https://os.phil-opp.com/freestanding-rust-binary/>

## Current

<https://os.phil-opp.com/minimal-rust-kernel/>
