flamegraph *args:
    RUSTFLAGS="-C force-frame-pointers" cargo flamegraph {{args}}
