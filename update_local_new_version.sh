rm -rf ~/.cargo/bin/staple
cargo test -- --test-threads 1
cargo build
cp target/debug/staple ~/.cargo/bin/staple