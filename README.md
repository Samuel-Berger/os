# Samos

Operating system time!

Guide via <https://os.phil-opp.com/>

## Somethings that might be needed

```sh
rustup target add thumbv7em-none-eabihf # Build to an embedded target.
cargo build --target thumbv7em-none-eabihf
cargo rustc -- -C link-arg=-nostartfiles
```

## Done

<https://os.phil-opp.com/freestanding-rust-binary/>

## Current

<https://os.phil-opp.com/minimal-rust-kernel/>
