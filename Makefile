default: first-time-done export-maybe regen-db sqlite-cli import-maybe sqlite
first: first-time regen-db sqlite
first-time:
	mkdir db
	diesel migration run || rmdir db

export-maybe:
	if [ -f ./target/debug/floormap-cli ]; then if [ -f /tmp/floormap-export.json ]; then exit 1; else ./target/debug/floormap-cli export-database -o /tmp/floormap-export.json; fi; fi
import-maybe:
	if [ -f ./target/debug/floormap-cli ] && [ -f /tmp/floormap-export.json ]; then ./target/debug/floormap-cli import-database -i /tmp/floormap-export.json; mv /tmp/floormap-export.json "/tmp/floormap-export-$(shell date +%s).json"; fi

install-rust:
	sudo apt-get install -y make build-essential git jsbeautifier
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	sudo apt-get install -y libssl-dev pkg-config moreutils libpq-dev libsqlite3-dev
	. ~/.cargo/env && cargo install diesel_cli --no-default-features --features postgres,sqlite
	echo To finish Rust installation, please logout and login back
db/floor.sqlite3:
	diesel setup --database-url db/floor.sqlite3

regen-db: db/floor.sqlite3
	diesel migration redo --database-url db/floor.sqlite3
	rustfmt src/schema.rs
	./dev-scripts/print-model >src/models.rs
rustfmt:
	find src -name '*.rs' -exec rustfmt {} \;

sqlite-cli:
	cargo build --features floormap_sqlite --bin floormap-cli || if [ -f ./target/debug/floormap-cli ]; then ./target/debug/floormap-cli import-database -i /tmp/floormap-export.json; cp /tmp/floormap-export.json "/tmp/floormap-export-$(shell date +%s).json"; fi

sqlite:
	cargo build --features floormap_sqlite
clippy-sqlite:
	cargo clippy --features floormap_sqlite
js-beautify:
	find templates -name '*.mustache' -exec js-beautify -r {} \;
cute: rustfmt js-beautify




