build-HelloRustFunction:
	cargo build --release --target x86_64-unknown-linux-musl

	# バイナリをbootstrapにして、Artifacts Directoryに格納する
	# Current Artifacts Directory : /workspace/stock_data/.aws-sam/build/HelloRustFunction
	cp ./target/x86_64-unknown-linux-musl/release/hello $(ARTIFACTS_DIR)/bootstrap