use crate::adapter::model::work::WorkTable;
use crate::kernel::model::artist::Artist;
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
}
