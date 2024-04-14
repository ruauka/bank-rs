run-server:
	cargo run -p server
	#cargo run -p server -- --host 0.0.0.0 --port 8000

fmt:
	cargo fmt

doc:
	cargo doc --open