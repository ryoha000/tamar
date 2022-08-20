use crate::adapter::persistence::sqlite::Db;

pub async fn migration() {
    println!("call migration");
    let db = Db::new().await;
    let pool = db.0.clone();

    let sqls = get_migration_sqls();
    for sql in sqls.iter() {
        sqlx::query(sql).execute(&*pool).await.unwrap();
    }

    println!("end migration");
}

fn get_migration_sqls() -> Vec<String> {
    let artist = "
CREATE TABLE IF NOT EXISTS artist (
	id varchar(36) primary key,
	name varchar(255) not null,
	created_at datetime not null,
	updated_at datetime not null
);
    "
    .to_string();

    return vec![artist];
}
