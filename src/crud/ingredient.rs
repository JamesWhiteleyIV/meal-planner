use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub id: Option<i64>,
    pub name: String,
    pub unit: String, // g/ml/etc
    pub amount: f32,
    pub calories_kcal: f32,
    pub protein_g: f32,
    pub carbohydrates_g: f32,
    pub sugar_g: f32,
    pub fat_g: f32,
    pub saturated_fat_g: f32,
    pub fiber_g: f32,
    pub potassium_mg: f32,
    pub sodium_mg: f32,
    pub cholesterol_mg: f32,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct IngredientSimple {
    pub id: Option<i64>,
    pub name: String,
}

pub async fn create(pool: &Pool<Sqlite>, ingredient: &Ingredient) -> Result<i64> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO ingredients (
            name,
            unit,
            amount,
            calories_kcal,
            protein_g,
            carbohydrates_g,
            sugar_g,
            fat_g,
            saturated_fat_g,
            fiber_g,
            potassium_mg,
            sodium_mg,
            cholesterol_mg
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&ingredient.name)
    .bind(&ingredient.unit)
    .bind(ingredient.amount)
    .bind(ingredient.calories_kcal)
    .bind(ingredient.protein_g)
    .bind(ingredient.carbohydrates_g)
    .bind(ingredient.sugar_g)
    .bind(ingredient.fat_g)
    .bind(ingredient.saturated_fat_g)
    .bind(ingredient.fiber_g)
    .bind(ingredient.potassium_mg)
    .bind(ingredient.sodium_mg)
    .bind(ingredient.cholesterol_mg)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn read(pool: &Pool<Sqlite>) -> Result<Vec<Ingredient>> {
    let ingredients = sqlx::query_as::<_, Ingredient>(r#"SELECT * FROM ingredients"#)
        .fetch_all(pool)
        .await?;

    Ok(ingredients)
}

pub async fn read_by_search_string(
    pool: &Pool<Sqlite>,
    search_string: &str,
) -> Result<Vec<IngredientSimple>> {
    let ingredients = sqlx::query_as::<_, IngredientSimple>(
        r#"SELECT id, name FROM ingredients WHERE name LIKE '%' || $1 || '%'"#,
    )
    .bind(search_string)
    .fetch_all(pool)
    .await?;

    Ok(ingredients)
}
pub async fn read_one(pool: &Pool<Sqlite>, id: i64) -> Result<Ingredient> {
    let ingredient = sqlx::query_as::<_, Ingredient>(
        r#"
        SELECT * 
        FROM ingredients
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(ingredient)
}

#[cfg(test)]
mod tests {
    use crate::crud::{self, get_connection_pool, ingredient::*};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_create() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_filename = temp_file.path();
        let pool = get_connection_pool(temp_filename.to_str().unwrap()).await;

        crud::create_tables(&pool).await.unwrap();
        crud::populate_tables(&pool).await.unwrap();
        let ingredient = Ingredient {
            id: None,
            name: "test_ingredient".to_string(),
            unit: "ml".to_string(),
            amount: 500.0,
            calories_kcal: 320.0,
            protein_g: 20.0,
            carbohydrates_g: 10.0,
            sugar_g: 0.1,
            fat_g: 11.0,
            saturated_fat_g: 22.0,
            fiber_g: 100.0,
            potassium_mg: 2000.0,
            sodium_mg: 2000.1,
            cholesterol_mg: 20000.0,
        };
        let ingredient_id = create(&pool, &ingredient).await.unwrap();
        let ingredient = read_one(&pool, ingredient_id).await.unwrap();
        assert!(ingredient.id.is_some());
        assert_eq!(ingredient.name, "test_ingredient".to_string());
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

        let search_string = "beef";
        let ingredients = read_by_search_string(&pool, search_string).await.unwrap();
        dbg!(&ingredients);
        // 4 different leaness of ground beef
        assert_eq!(ingredients.len(), 4);
    }
}
