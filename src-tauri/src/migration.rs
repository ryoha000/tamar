use crate::adapter::persistence::sqlite::Db;

pub async fn migration() {
    println!("call migration");
    let db = Db::new().await;
    let pool = db.0.clone();

    let sql = "
CREATE TABLE IF NOT EXISTS artist (
	id varchar(36) primary key,
	name varchar(255) not null,
	created_at datetime not null,
	updated_at datetime not null
);
    ";
    sqlx::query(sql).execute(&*pool).await.unwrap();
    println!("end migration");
}
