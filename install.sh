#!/bin/sh

RUSTC=$(which rustc)
if [ "x${RUSTC}" = "x" ]; then
  sudo apt-get update
  sudo apt-get install -y git
  git clone https://ayourtch@github.com/ayourtch/rsp10auth
  git clone https://github.com/ayourtch/pg2sqlite2pg
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  echo Now exit the shell, login, and restart the script.
  exit
fi

echo Installing dependent libraries
sudo apt-get install -y build-essential pkg-config libpq-dev libsqlite3-dev sqlite3 cmake libpci-dev libsensors4-dev libwrap0-dev libsnmp-dev
sudo apt-get -y install sqlite3 nginx graphviz pkg-config libssl-dev sudo pkg-config libsnmp-dev imagemagick poppler-utils jsbeautifier


DIESEL_CLI=$(which diesel)
if [ "x${DIESEL_CLI}" = "x" ]; then
  echo Installing diesel_cli
  cargo install diesel_cli --no-default-features --features sqlite,postgres
  cargo install diesel_cli_ext
fi


if [ ! -e .env ]; then
	echo "Making temporary .env for sqlite database"
	echo "DATABASE_URL=./db/floor.sqlite3" >.env
fi

if [ ! -e /secrets/floormap ]; then
	echo Making secrets and other arrangements..
	make first-time
fi
make sqlite

echo Reconfigure and restart nginx
sudo rm /etc/nginx/sites-enabled/default
sudo ln -s /home/ubuntu/floormap/configs/nginx-sites-enabled /etc/nginx/sites-enabled/default
sudo service nginx restart

echo installing the pg2sqlite2pg
(cd pg2sqlite2pg; cargo build && cp target/debug/export-* ~/.cargo/bin/)

echo install Postgresql
sudo apt-get install -y postgresql

. /secrets/floormap
echo Converting Sqlite3 to postgres
POSTGRES_DB_USER="$pg_user" SQLITE3_DB_URL=db/floor.sqlite3 export-sqlite3-to-postgres >/tmp/postgres-install.dat

echo Import converted db into postgres
./scripts/import-postgres-backup /tmp/postgres-install.dat

echo compiling rsp10auth
(cd rsp10auth; cargo build && if [ ! -e auth-db.sqlite3 ] ; then (cat db-dump/sqlite3-auth-db-dump.txt | sqlite3 auth-db.sqlite3); fi);

echo compiling postgres version
make postgres

echo setting the postgres url
echo "DATABASE_URL=${POSTGRES_DB_URL}" >.env

sudo mkdir -p /var/a3s/http
MYUSER=$(whoami)
sudo chown -R ${MYUSER} /var/a3s/http
mkdir -p /var/a3s/http/floor-plan-images
mkdir -p /var/a3s/http/uploads

# FIXME: fill in the .env inside rsp10auth directory
# FIXME: cp .secret to rsp10auth directory





