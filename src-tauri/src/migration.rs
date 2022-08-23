use crate::adapter::persistence::sqlite::Db;

pub const UNKNOWN_ARTIST_NAME: &str = "Unknown Artist";

pub async fn migration() {
    println!("start migration");

    let db = Db::new().await;
    let pool = db.0.clone();

    let sqls = get_migration_sqls();
    for sql in sqls.iter() {
        sqlx::query(sql).execute(&*pool).await.unwrap();
    }

    println!("end migration");
}

#[cfg(test)]
pub fn migration_sync(db: Db) {
    use tauri::async_runtime::block_on;

    let pool = db.0.clone();

    let sqls = get_migration_sqls();
    for sql in sqls.iter() {
        block_on(sqlx::query(sql).execute(&*pool)).unwrap();
    }
}

fn get_migration_sqls() -> Vec<String> {
    let artist = "
CREATE TABLE IF NOT EXISTS artist (
	id varchar(36) primary key,
	name varchar(255) not null UNIQUE,
	created_at datetime not null,
	updated_at datetime not null,
    UNIQUE(name)
);
    "
    .to_string();

    let work = "
CREATE TABLE IF NOT EXISTS work (
	id varchar(36) primary key,
	title varchar(255) not null,
    artist_id varchar(36) not null,
	created_at datetime not null,
	updated_at datetime not null,
    UNIQUE(title, artist_id)
);
    "
    .to_string();

    let work_title_index =
        "CREATE INDEX IF NOT EXISTS work_title_index ON work(title);".to_string();
    let work_artist_id_index =
        "CREATE INDEX IF NOT EXISTS work_artist_id_index ON work(artist_id);".to_string();
    let work_updated_at_index =
        "CREATE INDEX IF NOT EXISTS work_updated_at_index ON work(updated_at);".to_string();
    let work_artist_id_updated_at_index =
        "CREATE INDEX IF NOT EXISTS work_artist_id_updated_at_index ON work(artist_id, updated_at);".to_string();
    let work_title_updated_at_index =
        "CREATE INDEX IF NOT EXISTS work_title_updated_at_index ON work(title, updated_at);"
            .to_string();

    let tag = "
CREATE TABLE IF NOT EXISTS tag (
	id varchar(36) primary key,
	name varchar(255) not null UNIQUE,
	created_at datetime not null,
	updated_at datetime not null,
    UNIQUE(name)
);
    "
    .to_string();

    let work_tag_map = "
CREATE TABLE IF NOT EXISTS work_tag_map (
	id varchar(36) primary key,
    work_id varchar(36) not null,
    tag_id varchar(36) not null,
	created_at datetime not null,
	updated_at datetime not null,
    UNIQUE(work_id, tag_id)
);
    "
    .to_string();

    let insert_unknown_artist = format!("
    INSERT OR IGNORE INTO artist(id, name, created_at, updated_at) VALUES(\"01GAYXAS9G6YHP4BTZDFT360P7\", \"{}\", datetime(CURRENT_TIMESTAMP), datetime(CURRENT_TIMESTAMP))
    ", UNKNOWN_ARTIST_NAME);

    return vec![
        artist,
        work,
        work_artist_id_index,
        work_artist_id_updated_at_index,
        work_title_index,
        work_title_updated_at_index,
        work_updated_at_index,
        tag,
        work_tag_map,
        insert_unknown_artist,
    ];
}
