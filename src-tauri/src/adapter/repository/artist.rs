use crate::adapter::model::artist::ArtistTable;
use crate::kernel::{
    model::{
        artist::{Artist, NewArtist},
        Id,
    },
    repository::artist::ArtistRepository,
};
use async_trait::async_trait;
use sqlx::query_as;

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
    use crate::kernel::model::artist::{Artist, NewArtist};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::test_util::{random_string, get_test_db};
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
    fn test_find_artist_by_name_not_found() {
        let db = get_test_db();

        let found = find_artist_by_name(db, "りょは9999999".to_string());
        assert!(found.is_none());
    }

    fn insert_artist(db: Db, new_artist: NewArtist) {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.insert(new_artist)).unwrap()
    }

    fn find_artist(db: Db, id: Id<Artist>) -> Option<Artist> {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.find(&id)).unwrap()
    }

    fn find_artist_by_name(db: Db, name: String) -> Option<Artist> {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.find_by_name(name)).unwrap()
    }
}
