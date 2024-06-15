http-server:
	@echo "  >  Starting http-server..."
	@cargo run -p server
	#cargo run -p server -- --host 0.0.0.0 --port 8000

grpc-server:
	@echo "  >  Starting gRPC-server..."
	@cargo run -p server -- --protocol grpc

fmt:
	@echo "  >  Formatting code..."
	@cargo fmt
	#cargo fmt --all -- --check

lint:
	@echo "  >  Linting code..."
	@cargo clippy
	#cargo clippy -- -D warnings

doc:
	@cargo doc --open