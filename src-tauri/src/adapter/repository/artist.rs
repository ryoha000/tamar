use crate::adapter::model::artist::{ArtistTable, ArtistTableWithViewTime};
use crate::kernel::model::artist::{SearchAlsoUsingWorkArtist, UpdateNameArtist};
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

    async fn search_by_name(&self, name: &str) -> anyhow::Result<Vec<Artist>> {
        let pool = self.pool.0.clone();
        let artist_table = query_as::<_, ArtistTable>("select * from artist where name LIKE ?")
            .bind(format!("%{}%", name))
            .fetch_all(&*pool)
            .await?;
        Ok(artist_table
            .into_iter()
            .filter_map(|v| v.try_into().ok())
            .collect())
    }

    async fn search_also_using_work(
        &self,
        source: SearchAlsoUsingWorkArtist,
    ) -> anyhow::Result<Vec<Artist>> {
        // validation sort_col
        match &*source.sort_col {
            "name" | "updated_at" | "view_time" => {} // valid sort_col
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

        match &*source.sort_col {
            "name" => {
                builder.push("SELECT `artist`.`id` AS `id`, `artist`.`name` AS `name`, `artist`.`created_at` AS `created_at`, `artist`.`updated_at` AS `updated_at` FROM artist");
                // JOIN 先のテーブル
                let join_table = " SELECT artist_id, title FROM work GROUP BY artist_id ";
                let join_sql = format!(
                    " INNER JOIN ({}) AS work ON `work`.`artist_id` = `artist`.`id` ",
                    join_table
                );
                builder.push(join_sql);
            }
            "updated_at" => {
                builder.push("SELECT `artist`.`id` AS `id`, `artist`.`name` AS `name`, `artist`.`created_at` AS `created_at`, `artist`.`updated_at` AS `updated_at` FROM artist");
                // JOIN 先のテーブル
                let join_table =
                " SELECT artist_id, title, MAX(updated_at) AS latest FROM work GROUP BY artist_id ";
                let join_sql = format!(
                    " INNER JOIN ({}) AS work ON `work`.`artist_id` = `artist`.`id` ",
                    join_table
                );
                builder.push(join_sql);
            }
            "view_time" => {
                builder.push("SELECT `artist`.`id` AS `id`, `artist`.`name` AS `name`, `artist`.`created_at` AS `created_at`, `artist`.`updated_at` AS `updated_at`, `view_time` FROM artist");
                // JOIN 先のテーブル
                let history_join_table = " SELECT MAX(updated_at) AS view_time_per_work, work_id FROM work_history GROUP BY work_id ";
                let join_table = format!(" SELECT `work`.`id`, artist_id, title, MAX(view_time_per_work) AS view_time FROM work INNER JOIN ({}) AS history ON `history`.`work_id` = `work`.`id` GROUP BY artist_id ", history_join_table);
                let join_sql = format!(
                    " INNER JOIN ({}) AS work ON `work`.`artist_id` = `artist`.`id` ",
                    join_table
                );
                builder.push(join_sql);
            }
            _ => {}
        };

        if is_search {
            builder.push(" WHERE `work`.`title` LIKE ");
            builder.push_bind(format!("%{}%", source.text));

            builder.push(" OR `artist`.`name` LIKE ");
            builder.push_bind(format!("%{}%", source.text));
        }

        match &*source.sort_col {
            "name" => builder.push(format!(" ORDER BY `artist`.`name` {} ", sort_order_sql)),
            "updated_at" => builder.push(format!(" ORDER BY `work`.`latest` {} ", sort_order_sql)),
            "view_time" => builder.push(format!(" ORDER BY view_time {} ", sort_order_sql)),
            _ => anyhow::bail!("sort_col is invalid. sort_col: {}", source.sort_col),
        };

        builder.push(" LIMIT ");
        builder.push_bind(source.limit);

        builder.push(" OFFSET ");
        builder.push_bind(source.offset);

        let query = builder.build();
        let pool = self.pool.0.clone();
        let artist_table = query.fetch_all(&*pool).await?;

        match &*source.sort_col {
            "name" | "updated_at" => Ok(artist_table
                .into_iter()
                .filter_map(|v| ArtistTable::from_row(&v).ok()?.try_into().ok())
                .collect()),
            "view_time" => Ok(artist_table
                .into_iter()
                .filter_map(|v| ArtistTableWithViewTime::from_row(&v).ok()?.try_into().ok())
                .collect()),
            _ => anyhow::bail!("sort_col is invalid. sort_col: {}", source.sort_col),
        }
    }

    async fn insert(&self, source: NewArtist) -> anyhow::Result<()> {
        if source.name.len() == 0 {
            anyhow::bail!("name is required")
        }

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

    async fn update_name(&self, source: UpdateNameArtist) -> anyhow::Result<()> {
        if source.name.len() == 0 {
            anyhow::bail!("name is required")
        }
        let artist_table: ArtistTable = source.try_into()?;

        let pool = self.pool.0.clone();
        sqlx::query("UPDATE artist SET name = ?, updated_at = ? where id = ?")
            .bind(artist_table.name)
            .bind(artist_table.updated_at)
            .bind(artist_table.id)
            .execute(&*pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::kernel::model::artist::{
        Artist, NewArtist, SearchAlsoUsingWorkArtist, UpdateNameArtist,
    };
    use crate::kernel::model::work::{NewWork, Work};
    use crate::kernel::model::work_history::{NewWorkHistory, WorkHistory};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::work::WorkRepository;
    use crate::kernel::repository::work_history::WorkHistoryRepository;
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
    fn test_update_artist_name() {
        let db = get_test_db();
        let id = Ulid::new();
        let old_name = random_string();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(id), old_name.to_string()),
        );
        let found = find_artist(db.clone(), Id::new(id)).unwrap();

        assert_eq!(found.id.value, id);
        assert_eq!(found.name, old_name.to_string());

        let new_name = random_string();
        update_artist_name(
            db.clone(),
            UpdateNameArtist::new(Id::new(id), new_name.to_string()),
        );
        let found = find_artist(db.clone(), Id::new(id)).unwrap();

        assert_eq!(found.id.value, id);
        assert_eq!(found.name, new_name.to_string());
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
    fn test_search_artist_by_name() {
        let db = get_test_db();
        let id = Ulid::new();
        let text = random_string();
        let name = format!("{}-artist", &text);

        insert_artist(db.clone(), NewArtist::new(Id::new(id), name));
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(Ulid::new()), random_string()),
        );
        let found = search_artist_by_name(db, &text);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, id);
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
    fn test_search_artist_sort_by_history() {
        let db = get_test_db();
        let artist_id1 = Ulid::new();
        let artist_id2 = Ulid::new();
        let artist_id3 = Ulid::new();

        let insert_work_and_history = |artist_id: Ulid| {
            let id = Ulid::new();
            insert_work(
                db.clone(),
                NewWork::new(Id::new(id), random_string(), Id::new(artist_id)),
            );
            insert_work_history(
                db.clone(),
                NewWorkHistory::new(Id::new(Ulid::new()), Id::new(id)),
            );
        };

        // artist * 3
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id1), random_string()),
        );
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id2), random_string()),
        );
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id3), random_string()),
        );

        insert_work_and_history(artist_id1);
        insert_work_and_history(artist_id2);
        insert_work_and_history(artist_id3);
        insert_work_and_history(artist_id2);

        let source =
            SearchAlsoUsingWorkArtist::new(10, 0, "view_time".to_string(), true, "".to_string());

        let found = search_artist_also_using_work(db, source);
        assert_eq!(found.len(), 3);
        assert_eq!(found[0].id.value, artist_id2);
        assert_eq!(found[1].id.value, artist_id3);
        assert_eq!(found[2].id.value, artist_id1);
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

    fn search_artist_by_name(db: Db, name: &str) -> Vec<Artist> {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.search_by_name(name)).unwrap()
    }

    fn insert_work(db: Db, source: NewWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.insert(source)).unwrap()
    }

    fn insert_work_history(db: Db, source: NewWorkHistory) {
        let repository = DatabaseRepositoryImpl::<WorkHistory>::new(db);
        block_on(repository.insert(source)).unwrap()
    }

    fn search_artist_also_using_work(db: Db, source: SearchAlsoUsingWorkArtist) -> Vec<Artist> {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.search_also_using_work(source)).unwrap()
    }

    fn update_artist_name(db: Db, source: UpdateNameArtist) {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.update_name(source)).unwrap()
    }
}
