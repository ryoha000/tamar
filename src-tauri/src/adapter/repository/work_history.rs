use crate::adapter::model::work_history::WorkHistoryTable;
use crate::kernel::model::work::Work;
use crate::kernel::model::{
    work_history::{NewWorkHistory, WorkHistory},
    Id,
};
use crate::kernel::repository::work_history::WorkHistoryRepository;
use async_trait::async_trait;
use sqlx::query_as;

use super::DatabaseRepositoryImpl;

#[async_trait]
impl WorkHistoryRepository for DatabaseRepositoryImpl<WorkHistory> {
    async fn find(&self, id: &Id<WorkHistory>) -> anyhow::Result<Option<WorkHistory>> {
        let pool = self.pool.0.clone();
        let work_history_table =
            query_as::<_, WorkHistoryTable>("select * from work_history where id = ?")
                .bind(id.value.to_string())
                .fetch_one(&*pool)
                .await
                .ok();
        match work_history_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_latest(&self, id: &Id<Work>) -> anyhow::Result<Option<WorkHistory>> {
        let pool = self.pool.0.clone();
        let work_history_table = query_as::<_, WorkHistoryTable>(
            "select * from work_history where work_id = ? ORDER BY updated_at DESC",
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();
        match work_history_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewWorkHistory) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let work_history_table: WorkHistoryTable = source.try_into()?;
        let _ = sqlx::query(
            "insert into work_history (id, work_id, created_at, updated_at) values (?, ?, ?, ?)",
        )
        .bind(work_history_table.id)
        .bind(work_history_table.work_id)
        .bind(work_history_table.created_at)
        .bind(work_history_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::kernel::model::artist::{Artist, NewArtist};
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
    fn test_insert_work_history() {
        let db = get_test_db();
        let artist_id = Ulid::new();
        let work_id = Ulid::new();
        let name = random_string();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), name.to_string()),
        );
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
        );

        let id = Ulid::new();
        insert_work_history(
            db.clone(),
            NewWorkHistory {
                id: Id::new(id),
                work_id: Id::new(work_id),
            },
        );
        let found = find_work_history(db, Id::new(id)).unwrap();

        assert_eq!(found.id.value, id);
    }

    #[test]
    fn test_find_latest_work_history() {
        let db = get_test_db();
        let artist_id = Ulid::new();
        let work_id = Ulid::new();
        let name = random_string();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_id), name.to_string()),
        );
        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_id), random_string(), Id::new(artist_id)),
        );

        let id = Ulid::new();
        insert_work_history(
            db.clone(),
            NewWorkHistory {
                id: Id::new(id),
                work_id: Id::new(work_id),
            },
        );
        let id = Ulid::new();
        insert_work_history(
            db.clone(),
            NewWorkHistory {
                id: Id::new(id),
                work_id: Id::new(work_id),
            },
        );
        let found = find_latest_work_history(db, Id::new(work_id)).unwrap();

        assert_eq!(found.id.value, id);
    }

    fn insert_artist(db: Db, new_artist: NewArtist) {
        let repository = DatabaseRepositoryImpl::<Artist>::new(db);
        block_on(repository.insert(new_artist)).unwrap()
    }

    fn insert_work(db: Db, new_work: NewWork) {
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        block_on(repository.insert(new_work)).unwrap()
    }

    fn insert_work_history(db: Db, new_work_history: NewWorkHistory) {
        let repository = DatabaseRepositoryImpl::<WorkHistory>::new(db);
        block_on(repository.insert(new_work_history)).unwrap()
    }

    fn find_work_history(db: Db, id: Id<WorkHistory>) -> Option<WorkHistory> {
        let repository = DatabaseRepositoryImpl::<WorkHistory>::new(db);
        block_on(repository.find(&id)).unwrap()
    }

    fn find_latest_work_history(db: Db, id: Id<Work>) -> Option<WorkHistory> {
        let repository = DatabaseRepositoryImpl::<WorkHistory>::new(db);
        block_on(repository.find_latest(&id)).unwrap()
    }
}
