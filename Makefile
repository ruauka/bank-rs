run-server:
	cargo run -p server
	#cargo run -p server -- --host 0.0.0.0 --port 8000

fmt:
	cargo fmt

lint:
	cargo clippy

doc:
	cargo doc --open