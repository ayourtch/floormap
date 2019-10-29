default: regen-db sqlite
install-rust:
	sudo apt-get install -y make build-essential git
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	sudo apt-get install -y libssl-dev pkg-config moreutils libpq-dev libsqlite3-dev
	source ~/.cargo/env && cargo install diesel_cli --no-default-features --features postgres,sqlite
	mkdir db
	source ~/.cargo/env && diesel setup --database-url db/floor.sqlite3
	echo To finish Rust installation, please logout and login back
regen-db:
	diesel migration redo --database-url db/floor.sqlite3
	rustfmt src/schema.rs
	./dev-scripts/print-model >src/models.rs
rustfmt:
	find src -name '*.rs' -exec rustfmt {} \;

sqlite:
	cargo build --features floorplan_sqlite 




