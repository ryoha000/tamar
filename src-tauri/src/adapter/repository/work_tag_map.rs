use crate::adapter::model::work_tag_map::WorkTagMapTable;
use crate::kernel::model::tag::Tag;
use crate::kernel::model::work::Work;
use crate::kernel::{
    model::{
        work_tag_map::{NewWorkTagMap, WorkTagMap},
        Id,
    },
    repository::work_tag_map::WorkTagMapRepository,
};
use async_trait::async_trait;
use sqlx::query_as;

use super::DatabaseRepositoryImpl;

#[async_trait]
impl WorkTagMapRepository for DatabaseRepositoryImpl<WorkTagMap> {
    async fn find(&self, work_id: &Id<Work>, tag_id: &Id<Tag>) -> anyhow::Result<Vec<WorkTagMap>> {
        let pool = self.pool.0.clone();
        let work_tag_map_table = query_as::<_, WorkTagMapTable>(
            "select * from work_tag_map where work_id = ? AND tag_id = ?",
        )
        .bind(work_id.value.to_string())
        .bind(tag_id.value.to_string())
        .fetch_all(&*pool)
        .await
        .ok();
        match work_tag_map_table {
            Some(st) => Ok(st.into_iter().filter_map(|v| v.try_into().ok()).collect()),
            None => Ok(vec![]),
        }
    }

    async fn insert(&self, source: NewWorkTagMap) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let work_tag_map_table: WorkTagMapTable = source.try_into()?;
        let _ = sqlx::query(
            "insert into work_tag_map (id, work_id, tag_id, created_at, updated_at) values (?, ?, ?, ?, ?)",
        )
        .bind(work_tag_map_table.id)
        .bind(work_tag_map_table.work_id)
        .bind(work_tag_map_table.tag_id)
        .bind(work_tag_map_table.created_at)
        .bind(work_tag_map_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::kernel::model::artist::{Artist, NewArtist};
    use crate::kernel::model::tag::{NewTag, Tag};
    use crate::kernel::model::work::{NewWork, Work};
    use crate::kernel::model::work_tag_map::{NewWorkTagMap, WorkTagMap};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::tag::TagRepository;
    use crate::kernel::repository::work::WorkRepository;
    use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_work_tag_map() {
        let db = block_on(Db::new());

        let artist_ulid = Ulid::new();
        let work_ulid = Ulid::new();
        let tag_ulid = Ulid::new();
        let work_tag_map_ulid = Ulid::new();

        {
            let repository = DatabaseRepositoryImpl::<Artist>::new(db.clone());
            let _ = block_on(
                repository.insert(NewArtist::new(Id::new(artist_ulid), "りょは".to_string())),
            )
            .unwrap();
        }

        {
            let repository = DatabaseRepositoryImpl::<Work>::new(db.clone());
            let _ = block_on(repository.insert(NewWork::new(
                Id::new(work_ulid),
                "りょはのえっち本".to_string(),
                Id::new(artist_ulid),
            )))
            .unwrap();
        }

        {
            let repository = DatabaseRepositoryImpl::<Tag>::new(db.clone());
            let _ =
                block_on(repository.insert(NewTag::new(Id::new(tag_ulid), "えっち本".to_string())))
                    .unwrap();
        }

        {
            let repository = DatabaseRepositoryImpl::<WorkTagMap>::new(db.clone());
            let _ = block_on(repository.insert(NewWorkTagMap::new(
                Id::new(work_tag_map_ulid),
                Id::new(work_ulid),
                Id::new(tag_ulid),
            )))
            .unwrap();

            let found = block_on(repository.find(&Id::new(work_ulid), &Id::new(tag_ulid))).unwrap();

            assert_eq!(found[0].id.value, work_tag_map_ulid);
            assert_eq!(found[0].work_id.value, work_ulid);
            assert_eq!(found[0].tag_id.value, tag_ulid);
        }
    }
}
