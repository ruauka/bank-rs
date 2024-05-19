http-server:
	@cargo run -p server
	#cargo run -p server -- --host 0.0.0.0 --port 8000

grpc-server:
	@cargo run -p server -- --protocol grpc

fmt:
	@cargo fmt

lint:
	@cargo clippy

doc:
	@cargo doc --open