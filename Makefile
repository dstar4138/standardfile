hide:

run: build localite.db
	RUST_BACKTRACE=1 RUST_LOG=info cargo run

build:
	cargo build --verbose --all

localite.db:
	diesel database reset --database-url="localite.db"

travis: build
	cargo test --verbose --all
