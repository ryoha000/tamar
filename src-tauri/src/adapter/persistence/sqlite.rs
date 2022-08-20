use std::sync::Arc;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<Sqlite>>);

impl Db {
    pub async fn new() -> Db {
        let db_filename = "./tamar_sqlite.db3";
        let pool = SqlitePoolOptions::new()
            .max_connections(256)
            .connect(&format!("sqlite://{}?mode=rwc", db_filename))
            .await
            .map_err(|err| format!("{}\nfile: {}", err.to_string(), db_filename))
            .unwrap();

        Db(Arc::new(pool))
    }
}
