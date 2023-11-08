include .env
export

build:
	cargo build
release:
	cargo build --profile release
run:
	RUST_LOG=tower_http=trace RUST_BACKTRACE=0 cargo run

clean_local_registry:
	rm -rf ~/.cargo/registry

update: clean_local_registry
	rustup update
	cargo update

cleandb:
	sqlx database reset -y -D ${DATABASE_URL}