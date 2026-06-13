flamegraph *args:
    RUSTFLAGS="-C force-frame-pointers" cargo flamegraph {{args}}

run: 
    cargo run --release
