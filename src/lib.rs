#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

#[macro_use]
extern crate iron;

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate lazy_static;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use std::env;

use std::io::{Error, ErrorKind};

pub mod apiv1;
pub mod flextimestamp;
pub mod models;
pub mod schema;

#[macro_use]
pub mod template;

pub fn sqlite3_establish_connection() -> SqliteConnection {
    dotenv().ok();
    use diesel::connection::SimpleConnection;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    // connection.batch_execute("PRAGMA busy_timeout=20000;");
    connection
}

pub fn postgres_establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(feature = "floorplan_postgres")]
#[allow(non_camel_case_types)]
type DB_CONN_TYPE = PgConnection;
#[cfg(feature = "floorplan_postgres")]
pub const DB_TYPE_NAME: &str = "Postgres";
#[cfg(feature = "floorplan_postgres")]
pub fn uncached_establish_connection() -> DB_CONN_TYPE {
    postgres_establish_connection()
}

#[cfg(feature = "floorplan_sqlite")]
#[allow(non_camel_case_types)]
type DB_CONN_TYPE = SqliteConnection;
#[cfg(feature = "floorplan_sqlite")]
pub const DB_TYPE_NAME: &str = "Sqlite";
#[cfg(feature = "floorplan_sqlite")]
pub fn uncached_establish_connection() -> DB_CONN_TYPE {
    sqlite3_establish_connection()
}

pub fn create_db_pool() -> Pool<ConnectionManager<DB_CONN_TYPE>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<DB_CONN_TYPE>::new(database_url);
    let pool = Pool::builder()
        // match the pool size to be the same as Iron max threads
        // This is way too many:
        // .pool_size((8 * ::num_cpus::get() as u32))
        // .pool_size(4)
        .max_size(3)
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<DB_CONN_TYPE>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<DB_CONN_TYPE>>);

use diesel::connection::SimpleConnection;

impl DB {
    pub fn conn(&self) -> &DB_CONN_TYPE {
        let connection = &*self.0;
        // connection.batch_execute("PRAGMA busy_timeout=20000;");
        &*self.0
    }
}

macro_rules! db_insert_returning {
    ( $tbl: ident, $rec: expr, $retfield: ident, $typ: ty ) => {{
        let db = get_db();
        #[cfg(feature = "floormap_postgres")]
        let inserted_id: std::result::Result<$typ, diesel::result::Error> =
            diesel::insert_into(schema::$tbl::dsl::$tbl)
                .values($rec)
                .returning(schema::$tbl::dsl::$retfield)
                .get_result(db.conn());

        #[cfg(feature = "floormap_sqlite")]
        let inserted_id: std::result::Result<_, diesel::result::Error> = {
            panic!("sqlite3 backend does not support insertion fully. FIXME");
            diesel::insert_into(schema::$tbl::dsl::$tbl)
                .values($rec)
                .execute(db.conn());
            // FIXME SQLITE3
            Ok(-1)
        };
        inserted_id
    }};
}

pub fn get_db() -> DB {
    use std::thread;
    use std::time::Duration;

    loop {
        match DB_POOL.get() {
            Ok(conn) => break DB(conn),
            Err(e) => {
                println!(
                    "Could not get a conn! increase the # in the pool (err: {:?})",
                    e
                );
                thread::sleep(Duration::from_millis(10));
                continue;
            }
        }
    }
}
