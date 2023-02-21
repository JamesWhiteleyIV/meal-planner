use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, SqlitePool};
use sqlx::{FromRow, Pool, Sqlite};
use std::fs::File;
use std::io::BufReader;
use std::{fs, path::PathBuf};

use self::ingredient::Ingredient;
use self::tag::Tag;

pub mod ingredient;
pub mod recipe;
pub mod tag;

const DB_FILENAME: &str = "database.db";

pub async fn create_tables(pool: &Pool<Sqlite>) -> Result<()> {
    pool.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
    )
    .await?;

    pool.execute(
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

    pool.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            notes TEXT,
            instructions TEXT
        )",
    )
    .await?;

    pool.execute(
        "CREATE TABLE IF NOT EXISTS recipes_tags (
            id INTEGER PRIMARY KEY,
            recipe_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            FOREIGN KEY (recipe_id) REFERENCES recipes(id),
            FOREIGN KEY (tag_id) REFERENCES tags(id),
            CONSTRAINT unique_recipes_tags UNIQUE (recipe_id, tag_id)
        )",
    )
    .await?;

    pool.execute(
        "CREATE TABLE IF NOT EXISTS recipes_ingredients (
            id INTEGER PRIMARY KEY,
            recipe_id INTEGER NOT NULL,
            ingredient_id INTEGER NOT NULL,
            amount FLOAT NOT NULL,
            unit TEXT NOT NULL,
            FOREIGN KEY (recipe_id) REFERENCES recipes(id),
            FOREIGN KEY (ingredient_id) REFERENCES ingredients(id),
            CONSTRAINT unique_recipes_ingredients UNIQUE (recipe_id, ingredient_id)
        )",
    )
    .await?;

    Ok(())
}

pub async fn populate_tables(pool: &Pool<Sqlite>) -> Result<()> {
    // pre-populate db with ingredients
    let path = PathBuf::from("data/ingredients");
    let file_paths = get_json_files(path)?;
    for file_path in file_paths {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let ingredients: Vec<Ingredient> = serde_json::from_reader(reader)?;
        for ingredient in ingredients {
            ingredient::create(&pool, &ingredient).await?;
        }
    }

    // pre-populate db with tags
    let file_path = PathBuf::from("data/tags.json");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let tags: Vec<Tag> = serde_json::from_reader(reader)?;
    for tag in tags {
        tag::create(&pool, &tag).await?;
    }

    Ok(())
}

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
pub async fn get_connection_pool(db_filename: &str) -> Pool<Sqlite> {
    // Open a connection to the SQLite database
    SqlitePool::connect_with(
        SqliteConnectOptions::default()
            .filename(db_filename)
            .create_if_missing(true),
    )
    .await
    .unwrap()
}

// TODO
#[derive(FromRow, Serialize, Deserialize, Debug)]
struct MealPlan {
    id: i32,
    name: String, // meal-plan_{date}
    recipes: Vec<recipe::Recipe>,
}

fn create_meal_plan() {}

fn read_meal_plans() {}

// from trait to convert to grocery list

#[derive(Serialize, Deserialize, Debug)]
struct GroceryListIngredient {
    ingredient: ingredient::Ingredient,
    amount: f32,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
struct GroceryList {
    id: i32,
    name: String,
    ingredients: Vec<GroceryListIngredient>,
}

fn create_grocery_list() {}

fn read_grocery_lists() {}

/*
UnitConversion
id (primary key)
from_unit
to_unit
conversion_factor
*/
