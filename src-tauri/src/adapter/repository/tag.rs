use crate::adapter::model::tag::TagTable;
use crate::kernel::{
    model::{
        tag::{NewTag, Tag},
        Id,
    },
    repository::tag::TagRepository,
};
use async_trait::async_trait;
use sqlx::{query_as, FromRow};

use super::DatabaseRepositoryImpl;

#[async_trait]
impl TagRepository for DatabaseRepositoryImpl<Tag> {
    async fn find(&self, id: &Id<Tag>) -> anyhow::Result<Option<Tag>> {
        let pool = self.pool.0.clone();
        let tag_table = query_as::<_, TagTable>("select * from tag where id = ?")
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();
        match tag_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_by_ids(&self, ids: &Vec<Id<Tag>>) -> anyhow::Result<Vec<Tag>> {
        let pool = self.pool.0.clone();
        let mut builder = sqlx::query_builder::QueryBuilder::new("SELECT * FROM tag WHERE id IN (");
        let mut separated = builder.separated(", ");
        for id in ids.iter() {
            separated.push_bind(id.value.to_string());
        }
        separated.push_unseparated(")");
        let query = builder.build();
        let rows = query.fetch_all(&*pool).await?;
        Ok(rows
            .iter()
            .filter_map(|v| TagTable::from_row(v).ok())
            .filter_map(|v| v.try_into().ok())
            .collect())
    }

    async fn find_by_name(&self, name: String) -> anyhow::Result<Option<Tag>> {
        let pool = self.pool.0.clone();
        let tag_table = query_as::<_, TagTable>("select * from tag where name = ?")
            .bind(name)
            .fetch_one(&*pool)
            .await
            .ok();
        match tag_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewTag) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let tag_table: TagTable = source.try_into()?;
        let _ =
            sqlx::query("insert into tag (id, name, created_at, updated_at) values (?, ?, ?, ?)")
                .bind(tag_table.id)
                .bind(tag_table.name)
                .bind(tag_table.created_at)
                .bind(tag_table.updated_at)
                .execute(&*pool)
                .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::kernel::model::tag::{NewTag, Tag};
    use crate::kernel::model::Id;
    use crate::kernel::repository::tag::TagRepository;
    use crate::test_util::{random_string, get_test_db};
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_tag() {
        let db = get_test_db();
        let id = Ulid::new();
        let name = random_string();

        insert_tag(db.clone(), NewTag::new(Id::new(id), name.to_string()));
        let found = find_tag(db, Id::new(id)).unwrap();

        assert_eq!(found.id.value, id);
        assert_eq!(found.name, name.to_string());
    }

    #[test]
    fn test_find_tag_by_id() {
        let db = get_test_db();
        let id = Ulid::new();
        let name = random_string();

        insert_tag(db.clone(), NewTag::new(Id::new(id), name.to_string()));
        let found = find_tag_by_name(db, name.to_string()).unwrap();
        assert_eq!(found.id.value, id);
        assert_eq!(found.name, name.to_string());
    }

    #[test]
    fn test_find_tag_by_id_not_found() {
        let db = get_test_db();

        let found = find_tag_by_name(db, "りょは9999999".to_string());
        assert!(found.is_none());
    }

    #[test]
    fn test_find_tag_by_ids() {
        let db = get_test_db();

        let id1 = Ulid::new();
        let id2 = Ulid::new();

        insert_tag(db.clone(), NewTag::new(Id::new(id1), random_string()));
        insert_tag(db.clone(), NewTag::new(Id::new(id2), random_string()));

        let found = find_tag_by_ids(db, &vec![Id::new(id1), Id::new(id2)]);

        let mut is_exist_1 = false;
        for v in found.iter() {
            if v.id.value == id1 {
                is_exist_1 = true;
            } else {
                assert_eq!(v.id.value, id2)
            }
        }
        assert!(is_exist_1);
    }

    fn insert_tag(db: Db, new_tag: NewTag) {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.insert(new_tag)).unwrap()
    }

    fn find_tag(db: Db, id: Id<Tag>) -> Option<Tag> {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.find(&id)).unwrap()
    }

    fn find_tag_by_name(db: Db, name: String) -> Option<Tag> {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.find_by_name(name)).unwrap()
    }

    fn find_tag_by_ids(db: Db, ids: &Vec<Id<Tag>>) -> Vec<Tag> {
        let repository = DatabaseRepositoryImpl::new(db);
        block_on(repository.find_by_ids(ids)).unwrap()
    }
}
