use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
}

pub async fn create(pool: &Pool<Sqlite>, tag: &Tag) -> Result<i64> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO tags (
            name
        ) VALUES (?)",
    )
    .bind(&tag.name)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn read(pool: &Pool<Sqlite>) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(r#"SELECT * FROM tags"#)
        .fetch_all(pool)
        .await?;

    Ok(tags)
}

pub async fn read_by_search_string(pool: &Pool<Sqlite>, search_string: &str) -> Result<Vec<Tag>> {
    let tags =
        sqlx::query_as::<_, Tag>(r#"SELECT id, name FROM tags WHERE name LIKE '%' || $1 || '%'"#)
            .bind(search_string)
            .fetch_all(pool)
            .await?;

    Ok(tags)
}

pub async fn read_one(pool: &Pool<Sqlite>, id: i64) -> Result<Tag> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        SELECT * 
        FROM tags 
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(tag)
}

#[cfg(test)]
mod tests {
    use crate::crud::{self, get_connection_pool, tag::*};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_create() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_filename = temp_file.path();
        let pool = get_connection_pool(temp_filename.to_str().unwrap()).await;

        crud::create_tables(&pool).await.unwrap();
        crud::populate_tables(&pool).await.unwrap();

        let tag = Tag {
            id: None,
            name: "test_tag".to_string(),
        };
        let tag_id = create(&pool, &tag).await.unwrap();
        let tag = read_one(&pool, tag_id).await.unwrap();
        assert!(tag.id.is_some());
        assert_eq!(tag.name, "test_tag".to_string());
    }

    #[tokio::test]
    async fn test_read() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_filename = temp_file.path();
        let pool = get_connection_pool(temp_filename.to_str().unwrap()).await;

        crud::create_tables(&pool).await.unwrap();
        crud::populate_tables(&pool).await.unwrap();
        let items = read(&pool).await.unwrap();
        assert!(items.len() > 0);
    }

    #[tokio::test]
    async fn test_search_string_query() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_filename = temp_file.path();
        let pool = get_connection_pool(temp_filename.to_str().unwrap()).await;

        crud::create_tables(&pool).await.unwrap();
        crud::populate_tables(&pool).await.unwrap();

        let search_string = "veg";
        let tags = read_by_search_string(&pool, search_string).await.unwrap();
        // vegan, vegetarian, vegetables
        assert_eq!(tags.len(), 3);
    }
}
