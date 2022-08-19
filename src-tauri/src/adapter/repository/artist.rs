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
    use crate::kernel::model::artist::NewArtist;
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    // TODO later fix
    #[ignore]
    async fn test_insert_artist() {
        let db = Db::new().await;
        let repository = DatabaseRepositoryImpl::new(db);
        let id = Ulid::new();
        let _ = repository
            .insert(NewArtist::new(Id::new(id), "りょは".to_string()))
            .await
            .unwrap();
        let found = repository.find(&Id::new(id)).await.unwrap().unwrap();
        assert_eq!(found.id.value, id);
    }
}
