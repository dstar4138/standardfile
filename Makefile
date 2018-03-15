SHELL := /bin/bash

.PHONY: hide run build travis sqlite mysql clean

hide:

run: build-sqlite sqlite
	RUST_BACKTRACE=1 RUST_LOG=info cargo run --features sqlite

run-mysql: build-mysql mysql
	RUST_BACKTRACE=1 RUST_LOG=info cargo run --features mysql

build-sqlite:
	cargo build --verbose --all --manifest-path standardfile/Cargo.toml --features sqlite

build-mysql:
	cargo build --verbose --all --manifest-path standardfile/Cargo.toml --features mysql

sqlite: clean
	ln -s backend_sqlite/migrations migrations
	diesel database reset --database-url="${DB_PATH}"

mysql: clean
	ln -s backend_mysql/migrations migrations
	diesel database reset --database-url="mysql://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

travis: build-sqlite build-mysql
	cargo test --verbose --all --manifest-path standardfile/Cargo.toml --features sqlite
	cargo test --verbose --all --manifest-path standardfile/Cargo.toml --features mysql

clean:
	-rm migrations
