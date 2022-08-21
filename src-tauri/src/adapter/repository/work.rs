use crate::adapter::model::work::WorkTable;
use crate::kernel::model::artist::Artist;
use crate::kernel::model::work::SearchWork;
use crate::kernel::{
    model::{
        work::{NewWork, Work},
        Id,
    },
    repository::work::WorkRepository,
};
use async_trait::async_trait;
use sqlx::{query_as, FromRow};

use super::DatabaseRepositoryImpl;

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

        let is_use_where = source.title.len() != 0;

        let sort_order_sql;
        match source.sort_desc {
            true => sort_order_sql = "DESC",
            false => sort_order_sql = "ASC",
        }

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM work");
        if is_use_where {
            builder.push(" WHERE title LIKE ");
            builder.push_bind(format!("%{}%", source.title));
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

    async fn insert(&self, source: NewWork) -> anyhow::Result<()> {
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
}

#[cfg(test)]
mod test {
    use crate::kernel::model::artist::{Artist, NewArtist};
    use crate::kernel::model::work::{NewWork, SearchWork, Work};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::work::WorkRepository;
    use crate::test_util::random_string;
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_work() {
        let db = block_on(Db::new());
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
    fn test_find_work_by_title_and_artist() {
        let db = block_on(Db::new());
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
        let db = block_on(Db::new());
        let artist_id = Ulid::new();
        let title = random_string();

        let found = find_work_by_title_and_artist(db, title, &Id::new(artist_id));
        assert!(found.is_none());
    }

    #[test]
    fn test_search_work_not_found() {
        let db = block_on(Db::new());
        let source = SearchWork::new(10, 0, "updated_at".to_string(), true, random_string());

        let found = search(db, source).unwrap();
        assert!(found.is_empty());
    }

    #[test]
    fn test_search_work_with_no_title() {
        let db = block_on(Db::new());
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
        let db = block_on(Db::new());
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

    fn insert_artist(db: Db, new_artist: NewArtist) {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.insert(new_artist)).unwrap()
    }

    fn insert_work(db: Db, new_work: NewWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.insert(new_work)).unwrap()
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

    fn search(db: Db, source: SearchWork) -> anyhow::Result<Vec<Work>> {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.search(source))
    }
}
