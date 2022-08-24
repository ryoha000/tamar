use crate::adapter::model::work_tag_map::WorkTagMapTable;
use crate::kernel::model::tag::Tag;
use crate::kernel::model::work::Work;
use crate::kernel::model::work_tag_map::DeleteWorkTagMap;
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
    async fn find(
        &self,
        work_id: &Id<Work>,
        tag_id: &Id<Tag>,
    ) -> anyhow::Result<Option<WorkTagMap>> {
        let pool = self.pool.0.clone();
        let work_tag_map_table = query_as::<_, WorkTagMapTable>(
            "select * from work_tag_map where work_id = ? AND tag_id = ?",
        )
        .bind(work_id.value.to_string())
        .bind(tag_id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        match work_tag_map_table {
            Some(st) => Ok(st.try_into().ok()),
            _ => Ok(None),
        }
    }

    async fn find_by_work_id(&self, work_id: &Id<Work>) -> anyhow::Result<Vec<WorkTagMap>> {
        let pool = self.pool.0.clone();
        let work_tag_map_table =
            query_as::<_, WorkTagMapTable>("select * from work_tag_map where work_id = ?")
                .bind(work_id.value.to_string())
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

    async fn delete(&self, source: DeleteWorkTagMap) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let _ = sqlx::query("DELETE FROM work_tag_map WHERE work_id = ? AND tag_id = ?")
            .bind(source.work_id.value.to_string())
            .bind(source.tag_id.value.to_string())
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
    use crate::kernel::model::work_tag_map::{DeleteWorkTagMap, NewWorkTagMap, WorkTagMap};
    use crate::kernel::model::Id;
    use crate::kernel::repository::artist::ArtistRepository;
    use crate::kernel::repository::tag::TagRepository;
    use crate::kernel::repository::work::WorkRepository;
    use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
    use crate::test_util::{get_test_db, random_string};
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_work_tag_map() {
        let db = get_test_db();

        let artist_ulid = Ulid::new();
        let work_ulid = Ulid::new();
        let tag_ulid = Ulid::new();
        let work_tag_map_ulid = Ulid::new();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_ulid), random_string()),
        );

        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_ulid), random_string(), Id::new(artist_ulid)),
        );

        insert_tag(db.clone(), NewTag::new(Id::new(tag_ulid), random_string()));

        insert_work_tag_map(
            db.clone(),
            NewWorkTagMap::new(
                Id::new(work_tag_map_ulid),
                Id::new(work_ulid),
                Id::new(tag_ulid),
            ),
        );

        let found =
            find_work_tag_map_by_work_and_tag_id(db, &Id::new(work_ulid), &Id::new(tag_ulid))
                .unwrap();

        assert_eq!(found.id.value, work_tag_map_ulid);
        assert_eq!(found.work_id.value, work_ulid);
        assert_eq!(found.tag_id.value, tag_ulid);
    }

    #[test]
    fn test_find_work_tag_map_by_work_id() {
        let db = get_test_db();

        let artist_ulid = Ulid::new();
        let work_ulid = Ulid::new();
        let tag_ulid1 = Ulid::new();
        let tag_ulid2 = Ulid::new();
        let work_tag_map_ulid1 = Ulid::new();
        let work_tag_map_ulid2 = Ulid::new();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_ulid), random_string()),
        );

        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_ulid), random_string(), Id::new(artist_ulid)),
        );

        insert_tag(db.clone(), NewTag::new(Id::new(tag_ulid1), random_string()));
        insert_tag(db.clone(), NewTag::new(Id::new(tag_ulid2), random_string()));

        insert_work_tag_map(
            db.clone(),
            NewWorkTagMap::new(
                Id::new(work_tag_map_ulid1),
                Id::new(work_ulid),
                Id::new(tag_ulid1),
            ),
        );
        insert_work_tag_map(
            db.clone(),
            NewWorkTagMap::new(
                Id::new(work_tag_map_ulid2),
                Id::new(work_ulid),
                Id::new(tag_ulid2),
            ),
        );

        let found = find_work_tag_map_by_work_id(db, &Id::new(work_ulid));

        let mut is_exist_1 = false;
        for v in found.iter() {
            if v.id.value == work_tag_map_ulid1 {
                is_exist_1 = true;
                assert_eq!(v.work_id.value, work_ulid);
                assert_eq!(v.tag_id.value, tag_ulid1);
            } else {
                assert_eq!(v.id.value, work_tag_map_ulid2);
                assert_eq!(v.work_id.value, work_ulid);
                assert_eq!(v.tag_id.value, tag_ulid2);
            }
        }
        assert!(is_exist_1);
    }

    #[test]
    fn test_delete_work_tag_map() {
        let db = get_test_db();

        let artist_ulid = Ulid::new();
        let work_ulid = Ulid::new();
        let tag_ulid = Ulid::new();
        let work_tag_map_ulid = Ulid::new();

        insert_artist(
            db.clone(),
            NewArtist::new(Id::new(artist_ulid), random_string()),
        );

        insert_work(
            db.clone(),
            NewWork::new(Id::new(work_ulid), random_string(), Id::new(artist_ulid)),
        );

        insert_tag(db.clone(), NewTag::new(Id::new(tag_ulid), random_string()));

        insert_work_tag_map(
            db.clone(),
            NewWorkTagMap::new(
                Id::new(work_tag_map_ulid),
                Id::new(work_ulid),
                Id::new(tag_ulid),
            ),
        );

        // この時点ではまだある
        let found = find_work_tag_map_by_work_and_tag_id(
            db.clone(),
            &Id::<Work>::new(work_ulid),
            &Id::<Tag>::new(tag_ulid),
        );
        assert!(found.is_some());

        delete_work_tag_tag(
            db.clone(),
            DeleteWorkTagMap::new(Id::<Work>::new(work_ulid), Id::<Tag>::new(tag_ulid)),
        );

        // もうない
        let found = find_work_tag_map_by_work_and_tag_id(
            db,
            &Id::<Work>::new(work_ulid),
            &Id::<Tag>::new(tag_ulid),
        );
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

    fn insert_tag(db: Db, new_tag: NewTag) {
        let repository = DatabaseRepositoryImpl::<Tag>::new(db);
        block_on(repository.insert(new_tag)).unwrap()
    }

    fn insert_work_tag_map(db: Db, new_work_tag_map: NewWorkTagMap) {
        let repository = DatabaseRepositoryImpl::<WorkTagMap>::new(db);
        block_on(repository.insert(new_work_tag_map)).unwrap()
    }

    fn delete_work_tag_tag(db: Db, delete_work_tag_map: DeleteWorkTagMap) {
        let repository = DatabaseRepositoryImpl::<WorkTagMap>::new(db);
        block_on(repository.delete(delete_work_tag_map)).unwrap()
    }

    fn find_work_tag_map_by_work_id(db: Db, work_id: &Id<Work>) -> Vec<WorkTagMap> {
        let repository = DatabaseRepositoryImpl::<WorkTagMap>::new(db);
        block_on(repository.find_by_work_id(work_id)).unwrap()
    }

    fn find_work_tag_map_by_work_and_tag_id(
        db: Db,
        work_id: &Id<Work>,
        tag_id: &Id<Tag>,
    ) -> Option<WorkTagMap> {
        block_on(DatabaseRepositoryImpl::<WorkTagMap>::new(db).find(work_id, tag_id)).unwrap()
    }
}
