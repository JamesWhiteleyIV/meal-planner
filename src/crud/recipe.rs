use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Row, Sqlite};

use super::ingredient::Ingredient;
use super::tag::Tag;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeIngredient {
    pub ingredient: Ingredient,
    pub amount: f32,
    pub unit: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: Option<i64>,
    pub name: String,
    pub tags: Vec<Tag>,
    pub ingredients: Vec<RecipeIngredient>,
    pub notes: Vec<String>,
    pub instructions: Vec<String>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct RecipeSimple {
    pub id: Option<i64>,
    pub name: String,
}

pub async fn create(pool: &Pool<Sqlite>, recipe_name: &str) -> Result<i64> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO recipes (
            name
        ) VALUES (?)",
    )
    .bind(recipe_name)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn read(pool: &Pool<Sqlite>) -> Result<Vec<RecipeSimple>> {
    let recipes = sqlx::query_as::<_, RecipeSimple>(r#"SELECT id, name FROM recipes"#)
        .fetch_all(pool)
        .await?;

    Ok(recipes)
}

pub async fn read_by_tag_id(pool: &Pool<Sqlite>, tag_id: i64) -> Result<Vec<RecipeSimple>> {
    let recipes = sqlx::query_as::<_, RecipeSimple>(r#"SELECT recipes.id, recipes.name FROM recipes JOIN recipes_tags ON recipes.id = recipes_tags.recipe_id WHERE recipes_tags.tag_id = ?"#)
        .bind(tag_id)
        .fetch_all(pool)
        .await?;

    Ok(recipes)
}

pub async fn read_one(pool: &Pool<Sqlite>, recipe_id: i64) -> Result<Recipe> {
    let ingredients = get_recipe_ingredients(pool, recipe_id);
    let tags = get_recipe_tags(pool, recipe_id);
    let ingredients = ingredients.await?;
    let tags = tags.await?;

    let row = sqlx::query(
        r#"SELECT *
           FROM recipes
           WHERE id = ?"#,
    )
    .bind(recipe_id)
    .fetch_one(pool)
    .await?;

    let notes: String = row.get(2);
    let notes: Vec<String> = notes.split(";").map(|s| s.to_string()).collect();
    let instructions: String = row.get(3);
    let instructions: Vec<String> = instructions.split(";").map(|s| s.to_string()).collect();
    Ok(Recipe {
        id: Some(row.get(0)),
        name: row.get(1),
        tags,
        ingredients,
        notes,
        instructions,
    })
}

pub async fn add_recipe_ingredient(
    pool: &Pool<Sqlite>,
    recipe_id: i64,
    ingredient_id: i64,
    amount: f32,
    unit: &str,
) -> Result<i64> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO recipes_ingredients (
            recipe_id, 
            ingredient_id,
            amount,
            unit
        ) VALUES (?, ?, ?, ?)",
    )
    .bind(recipe_id)
    .bind(ingredient_id)
    .bind(amount)
    .bind(unit)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn add_recipe_tag(pool: &Pool<Sqlite>, recipe_id: i64, tag_id: i64) -> Result<i64> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO recipes_tags (
            recipe_id, 
            tag_id
        ) VALUES (?, ?)",
    )
    .bind(recipe_id)
    .bind(tag_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

async fn get_recipe_ingredients(
    pool: &Pool<Sqlite>,
    recipe_id: i64,
) -> Result<Vec<RecipeIngredient>> {
    let result = sqlx::query(
        r#"SELECT recipes_ingredients.amount, recipes_ingredients.unit, ingredients.*
           FROM recipes_ingredients
           JOIN ingredients ON recipes_ingredients.ingredient_id = ingredients.id
           WHERE recipes_ingredients.recipe_id = ?"#,
    )
    .bind(recipe_id)
    .map(|row: sqlx::sqlite::SqliteRow| {
        let ingredient = Ingredient {
            id: Some(row.get(2)),
            name: row.get(3),
            unit: row.get(4),
            amount: row.get(5),
            calories_kcal: row.get(6),
            protein_g: row.get(7),
            carbohydrates_g: row.get(8),
            sugar_g: row.get(9),
            fat_g: row.get(10),
            saturated_fat_g: row.get(11),
            fiber_g: row.get(12),
            potassium_mg: row.get(13),
            sodium_mg: row.get(14),
            cholesterol_mg: row.get(15),
        };
        RecipeIngredient {
            ingredient,
            amount: row.get(0),
            unit: row.get(1),
        }
    })
    .fetch_all(pool)
    .await?;

    Ok(result)
}

async fn get_recipe_tags(pool: &Pool<Sqlite>, recipe_id: i64) -> Result<Vec<Tag>> {
    let result = sqlx::query(
        r#"SELECT tags.id, tags.name
           FROM recipes_tags
           JOIN tags ON recipes_tags.tag_id = tags.id
           WHERE recipes_tags.recipe_id = ?"#,
    )
    .bind(recipe_id)
    .map(|row: sqlx::sqlite::SqliteRow| Tag {
        id: Some(row.get(0)),
        name: row.get(1),
    })
    .fetch_all(pool)
    .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::crud::{self, get_connection_pool, recipe::*};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_simple_crud() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_filename = temp_file.path();
        let pool = get_connection_pool(temp_filename.to_str().unwrap()).await;

        crud::create_tables(&pool).await.unwrap();
        crud::populate_tables(&pool).await.unwrap();
        let recipe_name = "test_recipe";
        let recipe_id = create(&pool, recipe_name).await.unwrap();

        add_recipe_ingredient(&pool, recipe_id, 1, 20.0, "ml")
            .await
            .unwrap();
        add_recipe_ingredient(&pool, recipe_id, 2, 1.0, "tsp")
            .await
            .unwrap();
        add_recipe_tag(&pool, recipe_id, 5).await.unwrap();
        add_recipe_tag(&pool, recipe_id, 7).await.unwrap();

        let _recipe = read_one(&pool, recipe_id).await.unwrap();
        let _recipes = read(&pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_tagging_query() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_filename = temp_file.path();
        let pool = get_connection_pool(temp_filename.to_str().unwrap()).await;

        crud::create_tables(&pool).await.unwrap();
        crud::populate_tables(&pool).await.unwrap();
        let tag_id = 2;

        let recipe_id = create(&pool, "test_recipe1").await.unwrap();
        add_recipe_tag(&pool, recipe_id, tag_id).await.unwrap();
        let recipe_id = create(&pool, "test_recipe2").await.unwrap();
        add_recipe_tag(&pool, recipe_id, tag_id).await.unwrap();
        let recipe_id = create(&pool, "test_recipe3").await.unwrap();
        add_recipe_tag(&pool, recipe_id, tag_id).await.unwrap();

        create(&pool, "should_not_get_these1").await.unwrap();
        create(&pool, "should_not_get_these2").await.unwrap();
        create(&pool, "should_not_get_these3").await.unwrap();

        let recipes = read_by_tag_id(&pool, tag_id).await.unwrap();
        assert_eq!(recipes.len(), 3);
    }
}
