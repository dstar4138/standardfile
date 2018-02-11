hide:

run: build localite.db
	cargo run

build:
	cargo build

localite.db:
	diesel database reset --database-url="localite.db"
