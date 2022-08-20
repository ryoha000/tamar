use crate::adapter::model::work::WorkTable;
use crate::kernel::{
    model::{
        work::{NewWork, Work},
        Id,
    },
    repository::work::WorkRepository,
};
use async_trait::async_trait;
use sqlx::query_as;

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
    use crate::kernel::model::work::{NewWork, Work};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::work::WorkRepository;
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_work() {
        let artist_id = Ulid::new();

        {
            let db = block_on(Db::new());
            let artist_repository = DatabaseRepositoryImpl::<Artist>::new(db);
            let _ = block_on(
                artist_repository.insert(NewArtist::new(Id::new(artist_id), "りょは".to_string())),
            )
            .unwrap();
        }

        let db = block_on(Db::new());
        let repository = DatabaseRepositoryImpl::<Work>::new(db);
        let id = Ulid::new();
        let _ = block_on(repository.insert(NewWork::new(
            Id::new(id),
            "りょはのえっち本".to_string(),
            Id::new(artist_id),
        )))
        .unwrap();
        let found = block_on(repository.find(&Id::new(id))).unwrap().unwrap();

        assert_eq!(found.id.value, id);
    }
}
