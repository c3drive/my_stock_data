## bin

Sample toml
```toml
[[bin]]
name = "stock"
path = "src/stock.rs"
```
sample Command
```bash
cargo run --bin stock 
cargo build --bin stock 
cargo build --bin stock --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/comment $(ARTIFACTS_DIR)/bootstrap
```