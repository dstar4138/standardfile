hide:

run: build localite.db
	RUST_BACKTRACE=1 cargo run

build:
	cargo build

localite.db:
	diesel database reset --database-url="localite.db"
