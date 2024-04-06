run-server:
	cd server && cargo run

fmt:
	cd server && cargo fmt && cd .. && cd client && cargo fmt

doc:
	cargo doc --open