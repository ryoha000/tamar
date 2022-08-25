use crate::adapter::model::work::WorkTable;
use crate::kernel::model::artist::Artist;
use crate::kernel::model::work::{
    NewerArtistIdWork, NewerTitleWork, SearchAroundTitleWork, SearchAroundUpdatedAtWork, SearchWork,
};
use crate::kernel::{
    model::{
        work::{NewWork, Work},
        Id,
    },
    repository::work::WorkRepository,
};
use async_trait::async_trait;
use sqlx::{query_as, FromRow, Sqlite};

use super::DatabaseRepositoryImpl;

fn get_search_around_query(
    builder: &mut sqlx::QueryBuilder<Sqlite>,
    limit: u16,
    is_before: bool,
    col: String,
    value: String,
) {
    builder.push(format!("SELECT * FROM work WHERE {} ", col));
    if is_before {
        builder.push(" < ");
    } else {
        builder.push(" > ");
    }
    builder.push_bind(format!("{}", value));

    builder.push(format!(" ORDER BY {} ", col));
    if is_before {
        builder.push(" DESC ");
    } else {
        builder.push(" ASC ");
    }

    builder.push(" LIMIT ");
    builder.push_bind(limit);
}

#[async_trait]
impl WorkRepository for DatabaseRepositoryImpl<Work> {
    async fn find(&self, id: &Id<Work>) -> anyhow::Result<Option<Work>> {
        let pool = self.pool.0.clone();
        let work_table = query_as::<_, WorkTable>("select * from work where id = ?")
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();
        match work_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_by_artist(&self, artist_id: &Id<Artist>) -> anyhow::Result<Vec<Work>> {
        let pool = self.pool.0.clone();
        let work_table = query_as::<_, WorkTable>("select * from work where artist_id = ?")
            .bind(artist_id.value.to_string())
            .fetch_all(&*pool)
            .await?;
        Ok(work_table
            .into_iter()
            .filter_map(|st| st.try_into().ok())
            .collect())
    }

    async fn find_by_title_and_artist(
        &self,
        title: String,
        artist_id: &Id<Artist>,
    ) -> anyhow::Result<Option<Work>> {
        let pool = self.pool.0.clone();
        let artist_table =
            query_as::<_, WorkTable>("select * from work where title = ? AND artist_id = ?")
                .bind(title)
                .bind(artist_id.value.to_string())
                .fetch_one(&*pool)
                .await
                .ok();
        match artist_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn search(&self, source: SearchWork) -> anyhow::Result<Vec<Work>> {
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
        if is_search {
            builder.push("SELECT `work`.`id` AS `id`, `work`.`title` AS `title`, `work`.`artist_id` AS `artist_id`, `work`.`created_at` AS `created_at`, `work`.`updated_at` AS `updated_at` FROM work");
            builder.push(" INNER JOIN artist ON `work`.`artist_id` = `artist`.`id` ");
            builder.push(" WHERE `work`.`title` LIKE ");
            builder.push_bind(format!("%{}%", source.text));

            builder.push(" OR `artist`.`name` LIKE ");
            builder.push_bind(format!("%{}%", source.text));
        } else {
            builder.push("SELECT * FROM work");
        }

        builder.push(format!(" ORDER BY {} {} ", source.sort_col, sort_order_sql));

        builder.push(" LIMIT ");
        builder.push_bind(source.limit);

        builder.push(" OFFSET ");
        builder.push_bind(source.offset);

        let query = builder.build();
        let pool = self.pool.0.clone();
        let work_table = query.fetch_all(&*pool).await?;

        Ok(work_table
            .into_iter()
            .filter_map(|v| WorkTable::from_row(&v).ok()?.try_into().ok())
            .collect())
    }

    async fn search_around_title(
        &self,
        source: SearchAroundTitleWork,
    ) -> anyhow::Result<Vec<Work>> {
        let mut builder = sqlx::QueryBuilder::new("");
        get_search_around_query(
            &mut builder,
            source.limit,
            source.is_before,
            "title".to_string(),
            source.title,
        );
        let pool = self.pool.0.clone();
        let work_table = builder.build().fetch_all(&*pool).await?;

        Ok(work_table
            .into_iter()
            .filter_map(|v| WorkTable::from_row(&v).ok()?.try_into().ok())
            .collect())
    }

    async fn search_around_updated_at(
        &self,
        source: SearchAroundUpdatedAtWork,
    ) -> anyhow::Result<Vec<Work>> {
        let mut builder = sqlx::QueryBuilder::new("");
        get_search_around_query(
            &mut builder,
            source.limit,
            source.is_before,
            "updated_at".to_string(),
            source.updated_at.to_string(),
        );

        let pool = self.pool.0.clone();
        let work_table = builder.build().fetch_all(&*pool).await?;

        Ok(work_table
            .into_iter()
            .filter_map(|v| WorkTable::from_row(&v).ok()?.try_into().ok())
            .collect())
    }

    async fn insert(&self, source: NewWork) -> anyhow::Result<()> {
        if source.title.len() == 0 {
            anyhow::bail!("title is required")
        }
        let pool = self.pool.0.clone();
        let work_table: WorkTable = source.try_into()?;
        let _ = sqlx::query(
            "insert into work (id, title, artist_id, created_at, updated_at) values (?, ?, ?, ?, ?)",
        )
        .bind(work_table.id)
        .bind(work_table.title)
        .bind(work_table.artist_id)
        .bind(work_table.created_at)
        .bind(work_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(())
    }

    async fn update_title(&self, source: NewerTitleWork) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        sqlx::query("UPDATE work SET title = ? where id = ?")
            .bind(source.title)
            .bind(source.id.value.to_string())
            .execute(&*pool)
            .await
            .ok();

        Ok(())
    }

    async fn update_artist_id(&self, source: NewerArtistIdWork) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        sqlx::query("UPDATE work SET artist_id = ? where id = ?")
            .bind(source.artist_id.value.to_string())
            .bind(source.id.value.to_string())
            .execute(&*pool)
            .await
            .ok();

        Ok(())
    }

    async fn delete(&self, id: &Id<Work>) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let _ = sqlx::query("DELETE FROM work WHERE id = ?")
            .bind(id.value.to_string())
            .execute(&*pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use core::time;
    use std::thread;

    use crate::kernel::model::artist::{Artist, NewArtist};
    use crate::kernel::model::work::{
        NewWork, NewerArtistIdWork, NewerTitleWork, SearchAroundTitleWork,
        SearchAroundUpdatedAtWork, SearchWork, Work,
    };
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::work::WorkRepository;
    use crate::test_util::{get_test_db, random_string};
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_work() {
        let db = get_test_db();
        let artist_id = Ulid::new();

        {
            let artist_name = random_string();
            insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        }

        {
            let work_id = Ulid::new();
            let title = random_string();
            insert_work(
                db.clone(),
                NewWork::new(Id::new(work_id), title.clone(), Id::new(artist_id)),
            );

            let found = find_work(db, Id::new(work_id)).unwrap();
            assert_eq!(found.id.value, work_id);
            assert_eq!(found.title, title);
        }
    }

    #[test]
    fn test_update_work_title() {
        let db = get_test_db();
        let artist_id = Ulid::new();

        let artist_name = random_string();
        insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));

        let work_id = Ulid::new();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
        );
        let new_title = random_string();

        let found = find_work(db.clone(), Id::new(work_id)).unwrap();
        assert_eq!(found.id.value, work_id);
        // まだ変わってない
        assert_ne!(found.title, new_title);

        update_work_title(
            db.clone(),
            NewerTitleWork::new(Id::new(work_id), new_title.clone()),
        );

        let found = find_work(db.clone(), Id::new(work_id)).unwrap();
        assert_eq!(found.id.value, work_id);
        // 変わった
        assert_eq!(found.title, new_title);
    }

    #[test]
    fn test_update_work_artist_id() {
        let db = get_test_db();

        let old_artist_id = Ulid::new();
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(old_artist_id), random_string()),
        );

        let new_artist_id = Ulid::new();
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(new_artist_id), random_string()),
        );

        let work_id = Ulid::new();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(old_artist_id)),
        );

        let found = find_work(db.clone(), Id::new(work_id)).unwrap();
        assert_eq!(found.id.value, work_id);
        // まだ変わってない
        assert_ne!(found.artist_id.value, new_artist_id);

        update_work_artist_id(
            db.clone(),
            NewerArtistIdWork::new(Id::new(work_id), Id::new(new_artist_id)),
        );

        let found = find_work(db.clone(), Id::new(work_id)).unwrap();
        assert_eq!(found.id.value, work_id);
        // 変わった
        assert_eq!(found.artist_id.value, new_artist_id);
    }

    #[test]
    fn test_find_work_by_title_and_artist() {
        let db = get_test_db();
        let artist_id = Ulid::new();

        {
            let artist_name = random_string();
            insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        }

        {
            let work_id = Ulid::new();
            let title = random_string();
            insert_work(
                db.clone(),
                NewWork::new(Id::new(work_id), title.clone(), Id::new(artist_id)),
            );

            let found =
                find_work_by_title_and_artist(db, title.clone(), &Id::new(artist_id)).unwrap();
            assert_eq!(found.id.value, work_id);
            assert_eq!(found.title, title);
        }
    }

    #[test]
    fn test_find_work_by_title_and_artist_not_found() {
        let db = get_test_db();
        let artist_id = Ulid::new();
        let title = random_string();

        let found = find_work_by_title_and_artist(db, title, &Id::new(artist_id));
        assert!(found.is_none());
    }

    #[test]
    fn test_find_work_by_artist() {
        let db = get_test_db();

        let artist_id = Ulid::new();
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), random_string()),
        );
        let work_id = Ulid::new();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
        );

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

        let found = find_work_by_artist(db, &Id::new(artist_id));

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, work_id);
    }

    #[test]
    fn test_search_work_not_found() {
        let db = get_test_db();
        let source = SearchWork::new(10, 0, "updated_at".to_string(), true, random_string());

        let found = search(db, source).unwrap();
        assert!(found.is_empty());
    }

    #[test]
    fn test_search_work_with_no_title() {
        let db = get_test_db();
        let source = SearchWork::new(10, 0, "updated_at".to_string(), true, "".to_string());

        let artist_id = Ulid::new();

        {
            let artist_name = random_string();
            insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        }

        let work_id = Ulid::new();
        let title = random_string();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), title.clone(), Id::new(artist_id)),
        );

        let found = search(db, source).unwrap();
        assert!(!found.is_empty());
    }

    #[test]
    fn test_search_work_with_title() {
        let db = get_test_db();
        let title = random_string();
        let search_title = title
            .chars()
            .enumerate()
            .filter(|&(i, _)| i > 0) // unique
            .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
        let source = SearchWork::new(10, 0, "updated_at".to_string(), true, search_title);

        let artist_id = Ulid::new();

        {
            let artist_name = random_string();
            insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        }

        let work_id = Ulid::new();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), title.clone(), Id::new(artist_id)),
        );

        let found = search(db, source).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, work_id);
        assert_eq!(found[0].title, title);
    }

    #[test]
    fn test_search_work_with_name() {
        let db = get_test_db();
        let artist_name = random_string();
        let search_name = artist_name
            .chars()
            .enumerate()
            .filter(|&(i, _)| i > 0) // unique
            .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
        let source = SearchWork::new(10, 0, "updated_at".to_string(), true, search_name);

        let artist_id = Ulid::new();
        insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));

        let work_id = Ulid::new();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
        );

        let found = search(db, source).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, work_id);
    }

    #[test]
    fn test_search_work_with_title_name() {
        let db = get_test_db();
        let base = random_string();
        let name = format!("{}-artist", &base);
        let title = format!("{}-title", &base);

        let mut expected_ids = vec![];

        let source = SearchWork::new(10, 0, "updated_at".to_string(), true, base);

        // 検索に引っかかる artist_id
        let artist_id = Ulid::new();
        insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), name));

        // 検索に引っかからない artist_id
        let artist_fake_id = Ulid::new();
        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_fake_id), random_string()),
        );

        // 検索に引っかかる
        let work_id = Ulid::new();
        expected_ids.push(work_id.clone());
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
        );

        // 検索に引っかかる
        let work_id = Ulid::new();
        expected_ids.push(work_id.clone());
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), title, Id::new(artist_fake_id)),
        );

        // 検索に引っかからない
        let work_id = Ulid::new();
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_fake_id)),
        );

        let found = search(db, source).unwrap();
        assert_eq!(found.len(), 2);
        // sort_desc -> true なので逆順
        assert_eq!(found[0].id.value, expected_ids[1]);
        assert_eq!(found[1].id.value, expected_ids[0]);
    }

    #[test]
    fn test_search_work_around_title() {
        let db = get_test_db();
        let title_base = random_string();
        let titles = vec![
            format!("{}-1", title_base),
            format!("{}-2", title_base),
            format!("{}-3", title_base),
        ];

        let artist_id = Ulid::new();
        {
            let artist_name = random_string();
            insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        }

        for title in titles.iter() {
            let work_id = Ulid::new();
            insert_work(
                db.clone(),
                NewWork::new(Id::new(work_id), (*title).clone(), Id::new(artist_id)),
            );
        }

        let source = SearchAroundTitleWork::new(10, true, titles[1].clone());
        let found = search_around_title(db.clone(), source).unwrap();
        assert_eq!(found[0].title, titles[0]);

        let source = SearchAroundTitleWork::new(10, false, titles[1].clone());
        let found = search_around_title(db, source).unwrap();
        assert_eq!(found[0].title, titles[2]);
    }

    #[test]
    fn test_search_work_around_updated_at() {
        let db = get_test_db();

        let artist_id = Ulid::new();
        {
            let artist_name = random_string();
            insert_artist(db.clone(), NewArtist::new(Id::new(artist_id), artist_name));
        }

        let mut updated_at_vec = vec![];
        for _ in 0..3 {
            let work_id = Ulid::new();
            insert_work(
                db.clone(),
                NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
            );
            updated_at_vec.push(find_work(db.clone(), Id::new(work_id)).unwrap().updated_at);
            let ten_millis = time::Duration::from_millis(100);
            thread::sleep(ten_millis);
        }

        let source = SearchAroundUpdatedAtWork::new(10, true, updated_at_vec[1].clone());
        let found = search_around_updated_at(db.clone(), source).unwrap();

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].updated_at, updated_at_vec[0]);

        let source = SearchAroundUpdatedAtWork::new(10, false, updated_at_vec[1].clone());
        let found = search_around_updated_at(db, source).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].updated_at, updated_at_vec[2]);
    }

    fn insert_artist(db: Db, new_artist: NewArtist) {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.insert(new_artist)).unwrap()
    }

    fn insert_work(db: Db, new_work: NewWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.insert(new_work)).unwrap()
    }

    fn update_work_title(db: Db, newer_work: NewerTitleWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.update_title(newer_work)).unwrap()
    }

    fn update_work_artist_id(db: Db, newer_work: NewerArtistIdWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.update_artist_id(newer_work)).unwrap()
    }

    fn find_work(db: Db, id: Id<Work>) -> Option<Work> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.find(&id)).unwrap()
    }

    fn find_work_by_title_and_artist(
        db: Db,
        title: String,
        artist_id: &Id<Artist>,
    ) -> Option<Work> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.find_by_title_and_artist(title, artist_id)).unwrap()
    }

    fn find_work_by_artist(db: Db, artist_id: &Id<Artist>) -> Vec<Work> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.find_by_artist(artist_id)).unwrap()
    }

    fn search(db: Db, source: SearchWork) -> anyhow::Result<Vec<Work>> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.search(source))
    }

    fn search_around_title(db: Db, source: SearchAroundTitleWork) -> anyhow::Result<Vec<Work>> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.search_around_title(source))
    }

    fn search_around_updated_at(
        db: Db,
        source: SearchAroundUpdatedAtWork,
    ) -> anyhow::Result<Vec<Work>> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.search_around_updated_at(source))
    }
}
