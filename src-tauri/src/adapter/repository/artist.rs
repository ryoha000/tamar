use crate::adapter::model::artist::ArtistTable;
use crate::kernel::model::artist::SearchAlsoUsingWorkArtist;
use crate::kernel::{
    model::{
        artist::{Artist, NewArtist},
        Id,
    },
    repository::artist::ArtistRepository,
};
use async_trait::async_trait;
use sqlx::{query_as, FromRow};

use super::DatabaseRepositoryImpl;

#[async_trait]
impl ArtistRepository for DatabaseRepositoryImpl<Artist> {
    async fn find(&self, id: &Id<Artist>) -> anyhow::Result<Option<Artist>> {
        let pool = self.pool.0.clone();
        let artist_table = query_as::<_, ArtistTable>("select * from artist where id = ?")
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();
        match artist_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: String) -> anyhow::Result<Option<Artist>> {
        let pool = self.pool.0.clone();
        let artist_table = query_as::<_, ArtistTable>("select * from artist where name = ?")
            .bind(name)
            .fetch_one(&*pool)
            .await
            .ok();
        match artist_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn search_also_using_work(
        &self,
        source: SearchAlsoUsingWorkArtist,
    ) -> anyhow::Result<Vec<Artist>> {
        // validation sort_col
        match &*source.sort_col {
            "title" | "updated_at" => {} // valid sort_col
            _ => anyhow::bail!("sort_col is invalid. sort_col: {}", source.sort_col),
        }

        let is_search = source.text.len() != 0;

        let sort_order_sql;
        match source.sort_desc {
            true => sort_order_sql = "DESC",
            false => sort_order_sql = "ASC",
        }

        let mut builder = sqlx::QueryBuilder::new("");
        // sort は `work`.$sort_col でやるため JOIN は必要
        builder.push("SELECT `artist`.`id` AS `id`, `artist`.`name` AS `name`, `artist`.`created_at` AS `created_at`, `artist`.`updated_at` AS `updated_at` FROM artist");

        // JOIN 先のテーブル
        let join_table = format!(
            "SELECT artist_id, title, MAX({}) AS latest FROM work GROUP BY artist_id",
            source.sort_col
        );
        let join_sql = format!(
            " INNER JOIN ({}) AS work ON `work`.`artist_id` = `artist`.`id` ",
            join_table
        );
        builder.push(join_sql);

        if is_search {
            builder.push(" WHERE `work`.`title` LIKE ");
            builder.push_bind(format!("%{}%", source.text));

            builder.push(" OR `artist`.`name` LIKE ");
            builder.push_bind(format!("%{}%", source.text));
        }

        builder.push(format!(" ORDER BY `work`.`latest` {} ", sort_order_sql));

        builder.push(" LIMIT ");
        builder.push_bind(source.limit);

        builder.push(" OFFSET ");
        builder.push_bind(source.offset);

        let query = builder.build();
        let pool = self.pool.0.clone();
        let artist_table = query.fetch_all(&*pool).await?;

        Ok(artist_table
            .into_iter()
            .filter_map(|v| ArtistTable::from_row(&v).ok()?.try_into().ok())
            .collect())
    }

    async fn insert(&self, source: NewArtist) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let artist_table: ArtistTable = source.try_into()?;
        let _ = sqlx::query(
            "insert into artist (id, name, created_at, updated_at) values (?, ?, ?, ?)",
        )
        .bind(artist_table.id)
        .bind(artist_table.name)
        .bind(artist_table.created_at)
        .bind(artist_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::kernel::model::artist::{Artist, NewArtist, SearchAlsoUsingWorkArtist};
    use crate::kernel::model::work::{NewWork, Work};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::work::WorkRepository;
    use crate::test_util::{get_test_db, random_string};
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_artist() {
        let db = get_test_db();
        let id = Ulid::new();
        let name = random_string();

        insert_artist(db.clone(), NewArtist::new(Id::new(id), name.to_string()));
        let found = find_artist(db, Id::new(id)).unwrap();

        assert_eq!(found.id.value, id);
        assert_eq!(found.name, name.to_string());
    }

    #[test]
    fn test_find_artist_by_name() {
        let db = get_test_db();
        let id = Ulid::new();
        let name = random_string();

        insert_artist(db.clone(), NewArtist::new(Id::new(id), name.to_string()));
        let found = find_artist_by_name(db, name.to_string()).unwrap();
        assert_eq!(found.id.value, id);
        assert_eq!(found.name, name.to_string());
    }

    #[test]
    fn test_search_artist_also_not_found() {
        let db = get_test_db();
        let artist_id = Ulid::new();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), random_string()),
        );
        insert_work(
            db.clone(),
            NewWork::new(Id::new(Ulid::new()), random_string(), Id::new(artist_id)),
        );

        let source =
            SearchAlsoUsingWorkArtist::new(10, 0, "updated_at".to_string(), true, random_string());

        let found = search_artist_also_using_work(db, source);
        assert!(found.is_empty());
    }

    #[test]
    fn test_search_artist_also_no_text() {
        let db = get_test_db();
        let artist_id = Ulid::new();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), random_string()),
        );
        insert_work(
            db.clone(),
            NewWork::new(Id::new(Ulid::new()), random_string(), Id::new(artist_id)),
        );

        let source =
            SearchAlsoUsingWorkArtist::new(10, 0, "updated_at".to_string(), true, "".to_string());

        let found = search_artist_also_using_work(db, source);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, artist_id);
    }

    #[test]
    fn test_search_artist_also_name() {
        let db = get_test_db();
        let artist_id = Ulid::new();
        let text = random_string();
        let artist_name = format!("{}-artist", text);

        insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        // ふたつ入れても結果はひとつ
        insert_work(
            db.clone(),
            NewWork::new(Id::new(Ulid::new()), random_string(), Id::new(artist_id)),
        );
        insert_work(
            db.clone(),
            NewWork::new(Id::new(Ulid::new()), random_string(), Id::new(artist_id)),
        );

        let source = SearchAlsoUsingWorkArtist::new(10, 0, "updated_at".to_string(), true, text);

        let found = search_artist_also_using_work(db, source);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, artist_id);
    }

    #[test]
    fn test_search_artist_also_both() {
        let db = get_test_db();
        let text = random_string();
        let artist_name = format!("{}-artist", text);
        let work_title = format!("{}-work", text);
        let mut expected_ids = vec![];

        // name に引っかかる場合
        let artist_id = Ulid::new();
        expected_ids.push(artist_id);
        insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        insert_work(
            db.clone(),
            NewWork::new(Id::new(Ulid::new()), random_string(), Id::new(artist_id)),
        );

        // title に引っかかる場合
        let artist_id = Ulid::new();
        expected_ids.push(artist_id);
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), random_string()),
        );
        insert_work(
            db.clone(),
            NewWork::new(Id::new(Ulid::new()), work_title, Id::new(artist_id)),
        );

        // name, title に引っかからない場合
        let artist_fake_id = Ulid::new();
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_fake_id), random_string()),
        );
        insert_work(
            db.clone(),
            NewWork::new(
                Id::new(Ulid::new()),
                random_string(),
                Id::new(artist_fake_id),
            ),
        );

        let source = SearchAlsoUsingWorkArtist::new(10, 0, "updated_at".to_string(), true, text);

        let found = search_artist_also_using_work(db, source);
        assert_eq!(found.len(), 2);
        // updated_at の desc だからひっくり返る
        assert_eq!(found[0].id.value, expected_ids[1]);
        assert_eq!(found[1].id.value, expected_ids[0]);
    }

    #[test]
    fn test_find_artist_by_name_not_found() {
        let db = get_test_db();

        let found = find_artist_by_name(db, "りょは9999999".to_string());
        assert!(found.is_none());
    }

    fn insert_artist(db: Db, new_artist: NewArtist) {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.insert(new_artist)).unwrap()
    }

    fn find_artist(db: Db, id: Id<Artist>) -> Option<Artist> {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.find(&id)).unwrap()
    }

    fn find_artist_by_name(db: Db, name: String) -> Option<Artist> {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.find_by_name(name)).unwrap()
    }

    fn insert_work(db: Db, source: NewWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.insert(source)).unwrap()
    }

    fn search_artist_also_using_work(db: Db, source: SearchAlsoUsingWorkArtist) -> Vec<Artist> {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.search_also_using_work(source)).unwrap()
    }
}
