use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, SqlitePool};
use anyhow::Result;
use std::{fs::{self, File}, path::PathBuf, io::BufReader};

const DB_FILENAME: &str = "database.db";

// Recursively search for all JSON files in a folder and return their paths
fn get_json_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // Recurse into subdirectory
                let sub_file_paths = get_json_files(entry_path)?;
                file_paths.extend(sub_file_paths);
            } else if let Some(ext) = entry_path.extension() {
                if ext == "json" {
                    // Add the JSON file to the list
                    file_paths.push(entry_path);
                }
            }
        }
    }

    Ok(file_paths)
}

// Return db connection for use in subsequent queries
pub async fn get_connection() -> Pool<Sqlite> {
    // Open a connection to the SQLite database
    SqlitePool::connect_with(SqliteConnectOptions::default().filename(DB_FILENAME).create_if_missing(true))
        .await
        .unwrap()
}


// Create tables, pre-populate db
pub async fn setup(conn: &Pool<Sqlite>) -> Result<()> {
   // create ingredients table
   conn.execute(
            "CREATE TABLE IF NOT EXISTS ingredients (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            unit TEXT NOT NULL,
            amount FLOAT NOT NULL,
            calories_kcal FLOAT NOT NULL,
            protein_g FLOAT NOT NULL,
            carbohydrates_g FLOAT NOT NULL,
            sugar_g FLOAT NOT NULL,
            fat_g FLOAT NOT NULL,
            saturated_fat_g FLOAT NOT NULL,
            fiber_g FLOAT NOT NULL,
            potassium_mg FLOAT NOT NULL,
            sodium_mg FLOAT NOT NULL,
            cholesterol_mg FLOAT NOT NULL
        )",
        )
        .await?;

   // create tags table
   conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        )
        .await?;


    // pre-populate db with ingredients
    let path = PathBuf::from("data/ingredients");
    let file_paths = get_json_files(path)?;
    for file_path in file_paths {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let ingredients: Vec<Ingredient> = serde_json::from_reader(reader)?;
        for ingredient in ingredients {
            Ingredient::create(&conn, &ingredient).await?;
        }
    }

    // pre-populate db with tags
    let file_path = PathBuf::from("data/tags.json");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let tags: Vec<Tag> = serde_json::from_reader(reader)?;
    for tag in tags {
        Tag::create(&conn, &tag).await?;
    }

Ok(())
}


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Ingredient {
    id: Option<i32>,
    name: String,
    unit: String, // g/ml/etc
    amount: f32,
    calories_kcal: f32,
    protein_g: f32,
    carbohydrates_g: f32,
    sugar_g: f32,
    fat_g: f32,
    saturated_fat_g: f32,
    fiber_g: f32,
    potassium_mg: f32,
    sodium_mg: f32,
    cholesterol_mg: f32,
}

impl Ingredient {

 pub async fn create(conn: &Pool<Sqlite>, ingredient: &Ingredient) -> Result<()> {
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
    .execute(conn)
    .await?;

    Ok(())
 }

 pub async fn read(conn: &Pool<Sqlite>) -> Result<Vec<Ingredient>> {
    let ingredients = sqlx::query_as::<_, Ingredient>(r#"SELECT * FROM ingredients"#)
        .fetch_all(conn)
        .await?;

    Ok(ingredients)
 }

 pub async fn read_one(conn: &Pool<Sqlite>, id: i32) -> Result<Ingredient> {
    let ingredient = sqlx::query_as::<_, Ingredient>(
        r#"
        SELECT * 
        FROM ingredients
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(conn)
    .await?;

    Ok(ingredient)
 }

}



#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Tag {
    id: Option<i32>,
    name: String,
}


impl Tag {

 pub async fn create(conn: &Pool<Sqlite>, tag: &Tag) -> Result<()> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO tags (
            name
        ) VALUES (?)",
    )
    .bind(&tag.name)
    .execute(conn)
    .await?;

    Ok(())
 }

 pub async fn read(conn: &Pool<Sqlite>) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(r#"SELECT * FROM tags"#)
        .fetch_all(conn)
        .await?;

    Ok(tags)
 }

 pub async fn read_one(conn: &Pool<Sqlite>, id: i32) -> Result<Tag> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        SELECT * 
        FROM tags 
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(conn)
    .await?;

    Ok(tag)
 }

}



#[derive(FromRow, Serialize, Deserialize, Debug)]
struct Recipe {
    id: i32,
    name: String,
}

fn create_recipe() {}

fn read_recipes() {}

#[derive(FromRow, Serialize, Deserialize, Debug)]
struct MealPlan {
    id: i32,
    name: String, // meal-plan_{date}
    recipes: Vec<Recipe>,
}

fn create_meal_plan() {}

fn read_meal_plans() {}

// from trait to convert to grocery list

#[derive(Serialize, Deserialize, Debug)]
struct GroceryListEntry {
    ingredient: Ingredient,
    amount: f32,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
struct GroceryList {
    id: i32,
    name: String,
    ingredients: Vec<GroceryListEntry>,
}

fn create_grocery_list() {}

fn read_grocery_lists() {}

/*
UnitConversion
id (primary key)
from_unit
to_unit
conversion_factor


RecipeTag
RecipeIngredient

*/
