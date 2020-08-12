rm -rf ~/.cargo/bin/staple
cargo test
cargo build
cp target/debug/staple ~/.cargo/bin/staple