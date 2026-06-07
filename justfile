flamegraph *args:
    RUSTFLAGS="-C force-frame-pointers" --output-format speedscope cargo flamegraph {{args}}
