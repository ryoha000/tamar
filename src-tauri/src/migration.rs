use crate::adapter::persistence::sqlite::Db;

pub async fn migration() {
    println!("call migration");
    let db = Db::new().await;
    let pool = db.0.clone();

    let sqls = get_create_table_sqls();
    for sql in sqls.iter() {
        sqlx::query(sql).execute(&*pool).await.unwrap();
    }

    println!("end migration");
}

fn get_create_table_sqls() -> Vec<String> {
    let artist = "
CREATE TABLE IF NOT EXISTS artist (
	id varchar(36) primary key,
	name varchar(255) not null,
	created_at datetime not null,
	updated_at datetime not null
);
    "
    .to_string();

    let work = "
CREATE TABLE IF NOT EXISTS work (
	id varchar(36) primary key,
	title varchar(255) not null,
    artist_id varchar(36) not null,
	created_at datetime not null,
	updated_at datetime not null
);
    "
    .to_string();

    return vec![artist, work];
}
