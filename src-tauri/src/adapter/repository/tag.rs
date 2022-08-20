use crate::adapter::model::tag::TagTable;
use crate::kernel::{
    model::{
        tag::{NewTag, Tag},
        Id,
    },
    repository::tag::TagRepository,
};
use async_trait::async_trait;
use sqlx::query_as;

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
    use crate::kernel::model::tag::NewTag;
    use crate::kernel::model::Id;
    use crate::kernel::repository::tag::TagRepository;
    use crate::test_util::random_string;
    use tauri::async_runtime::block_on;
    use ulid::Ulid;

    use crate::adapter::persistence::sqlite::Db;

    use super::DatabaseRepositoryImpl;

    #[test]
    fn test_insert_tag() {
        let db = block_on(Db::new());
        let repository = DatabaseRepositoryImpl::new(db);
        let id = Ulid::new();
        let name = random_string();
        let _ = block_on(repository.insert(NewTag::new(Id::new(id), name.clone()))).unwrap();
        let found = block_on(repository.find(&Id::new(id))).unwrap().unwrap();

        assert_eq!(found.id.value, id);
        assert_eq!(found.name, name);
    }
}
