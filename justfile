run target:
    cd {{target}} && cargo run

test target:
    cargo test -p {{target}}
