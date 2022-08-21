#[cfg(test)]
pub fn random_string() -> String {
    use ulid::Ulid;

    Ulid::new().to_string()
}

#[cfg(test)]
use crate::adapter::persistence::sqlite::Db;

#[cfg(test)]
pub fn get_test_db() -> Db {
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;
    use tauri::async_runtime::block_on;

    use crate::migration::migration_sync;

    std::fs::create_dir_all("test_db").unwrap();

    let db_filename = format!("./test_db/{}.db3", random_string());
    let pool = block_on(
        SqlitePoolOptions::new()
            .max_connections(256)
            .connect(&format!("sqlite://{}?mode=rwc", db_filename)),
    )
    .map_err(|err| format!("{}\nfile: {}", err.to_string(), db_filename))
    .unwrap();

    let db = Db(Arc::new(pool));
    migration_sync(db.clone());
    db
}
