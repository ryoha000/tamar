use crate::adapter::model::search_history::SearchHistoryTable;
use crate::kernel::model::{
    search_history::{NewSearchHistory, SearchHistory},
    Id,
};
use crate::kernel::repository::search_history::SearchHistoryRepository;
use async_trait::async_trait;
use sqlx::{query_as, FromRow};

use super::DatabaseRepositoryImpl;

#[async_trait]
impl SearchHistoryRepository for DatabaseRepositoryImpl<SearchHistory> {
    async fn find(&self, id: &Id<SearchHistory>) -> anyhow::Result<Option<SearchHistory>> {
        let pool = self.pool.0.clone();
        let search_history_table =
            query_as::<_, SearchHistoryTable>("select * from search_history where id = ?")
                .bind(id.value.to_string())
                .fetch_one(&*pool)
                .await
                .ok();
        match search_history_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn select_recent(&self, limit: u32) -> anyhow::Result<Vec<SearchHistory>> {
        let pool = self.pool.0.clone();
        let search_history_table = query_as::<_, SearchHistoryTable>(
            "select * from search_history ORDER BY updated_at DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&*pool)
        .await?;

        let mut res = vec![];
        for st in search_history_table {
            res.push(st.try_into()?);
        }
        Ok(res)
    }

    async fn insert(&self, source: NewSearchHistory) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let search_history_table: SearchHistoryTable = source.try_into()?;
        let _ = sqlx::query(
            "insert into search_history (id, value_id, value_type, created_at, updated_at) values (?, ?, ?, ?, ?)",
        )
        .bind(search_history_table.id)
        .bind(search_history_table.value_id)
        .bind(search_history_table.value_type)
        .bind(search_history_table.created_at)
        .bind(search_history_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::kernel::model::artist::{Artist, NewArtist};
    use crate::kernel::model::search_history::{
        NewSearchHistory, SearchHistory, SearchHistoryTypeEnum,
    };
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::search_history::SearchHistoryRepository;
    use crate::test_util::{get_test_db, random_string};
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_search_history() {
        let db = get_test_db();
        let artist_id = Ulid::new();
        let name = random_string();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), name.to_string()),
        );

        let id = Ulid::new();
        insert_search_history(
            db.clone(),
            NewSearchHistory {
                id: Id::new(id),
                value_id: artist_id.to_string(),
                value_type: SearchHistoryTypeEnum::artist,
            },
        );
        let found = find_search_history(db, Id::new(id)).unwrap();

        assert_eq!(found.id.value, id);
        assert_eq!(found.value_id, artist_id.to_string());
    }

    #[test]
    fn test_select_recent_search_history() {
        let db = get_test_db();
        let artist_id = Ulid::new();
        let name = random_string();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), name.to_string()),
        );

        let id = Ulid::new();
        insert_search_history(
            db.clone(),
            NewSearchHistory {
                id: Id::new(id),
                value_id: artist_id.to_string(),
                value_type: SearchHistoryTypeEnum::artist,
            },
        );

        let id = Ulid::new();
        insert_search_history(
            db.clone(),
            NewSearchHistory {
                id: Id::new(id),
                value_id: artist_id.to_string(),
                value_type: SearchHistoryTypeEnum::artist,
            },
        );
        let found = select_recent_search_history(db, 1);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id.value, id);
    }

    fn insert_artist(db: Db, new_artist: NewArtist) {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.insert(new_artist)).unwrap()
    }

    fn insert_search_history(db: Db, new_search_history: NewSearchHistory) {
        let repository = DatabaseRepositoryImpl::<SearchHistory>::new(db);
        block_on(repository.insert(new_search_history)).unwrap()
    }

    fn find_search_history(db: Db, id: Id<SearchHistory>) -> Option<SearchHistory> {
        let repository = DatabaseRepositoryImpl::<SearchHistory>::new(db);
        block_on(repository.find(&id)).unwrap()
    }

    fn select_recent_search_history(db: Db, limit: u32) -> Vec<SearchHistory> {
        let repository = DatabaseRepositoryImpl::<SearchHistory>::new(db);
        block_on(repository.select_recent(limit)).unwrap()
    }
}
