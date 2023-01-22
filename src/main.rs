// https://docs.rs/rusqlite/0.28.0/rusqlite/struct.Statement.html

use anyhow::Result;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};

pub const DATABASE_FILENAME: &str = "foodbuddy.db";


/*
#[derive(Debug)]
pub struct GroceryList {
    pub id: i64,
    pub ingredients: Vec<Ingredient>
}


impl GroceryList {
    pub fn add(&mut self, i: Ingredient)  {
        let mut found = false;
        for ingredient in self.ingredients.iter_mut() {
            if ingredient.food_item.name == i.food_item.name {
                ingredient.amount += i.amount;
            }
            found = true;
        }
        if !found {
            self.ingredients.push(i);
        } 
    }
}
*/


#[derive(Debug, Serialize, Deserialize)]
pub struct MealPlan {
    pub id: i64,
    pub name: String,
    //pub recipes: Vec<Recipe>  // recipe_meal_plan table
}
// get meal plans (Vec<MealPlan>)
// create meal plan (empty)
// add to meal plan (Recipe)

fn create_meal_plans_table(conn: &Connection) -> Result<()> {
    println!("creating meal_plans table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meal_plans (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL, 
        UNIQUE (name)
    )",
    (),)?;
    Ok(())
}

fn create_meal_plan_recipe_table(conn: &Connection) -> Result<()> {
    println!("creating meal_plan_recipe table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meal_plan_recipe (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        meal_plan_id INTEGER,
        recipe_id INTEGER,
        UNIQUE(meal_plan_id, recipe_id)
    )",
    (),)?;
    Ok(())
}

fn add_recipe_to_meal_plan(conn: &Connection, recipe_id: i64, meal_plan_id: i64) -> Result<()> {

    Ok(())
}


// create new empty meal plan with given name
fn create_meal_plan(conn: &Connection, name: String) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO food_items (name) VALUES (?1);",
        (name,))?;
    Ok(())
}

fn get_meal_plans(conn: &Connection) -> Result<Vec<MealPlan>> {
    //let mut stmt = conn.prepare("SELECT recipe_id name FROM meal_plans WHERE (?1) = ")?;
    let mut stmt = conn.prepare("SELECT id, name FROM meal_plans")?;
    let rows = stmt.query_map([], |row| {
        Ok(MealPlan {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}

fn get_meal_plan_recipes(conn: &Connection, meal_plan_id: i64) -> Result<()> {
    let rows = conn.execute(
        "SELECT recipe_id FROM meal_plan_recipe WHERE meal_plan_id=(?1);",
        (meal_plan_id,),
        )?;
    /*
    for row in rows.iter() {
        println!("{:#?}", row);
    }
    */
    println!("{:#?}", rows);


    /*
    let mut stmt = conn.prepare("SELECT recipe_id FROM meal_plan_recipe WHERE meal_plan_id=(?1)", &[meal_plan_id])?;
    let rows = stmt.query_map([], |row| {
        Ok(Recipe {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;
    */

    //Ok(rows.into_iter().flatten().collect())
    Ok(())
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Amount(f64);

 
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String, // e.g. carrot, potato
    pub ingredients: Vec<(Ingredient, Amount, unit::Unit)>
}

fn create_recipes_table(conn: &Connection) -> Result<()> {
    println!("creating recipes table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        UNIQUE(name)
    )",
    (), 
    )?;

    Ok(())
}

fn create_recipe_ingredient_table(conn: &Connection) -> Result<()> {
    println!("creating recipe_ingredient table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_ingredient (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        recipe_id INTEGER NOT NULL,
        ingredient_id INTEGER NOT NULL,
        unit_id INTEGER NOT NULL,
        amount FLOAT NOT NULL,
        UNIQUE(recipe_id, ingredient_id)
    )",
    (), 
    )?;

    Ok(())
}

fn add_ingredient_to_recipe(conn: &Connection, ingredient_id: i64, recipe_id: i64, unit_id: i64, amount: f64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO recipe_ingredient (ingredient_id, recipe_id, unit_id, amount) VALUES (?1, ?2, ?3, ?4);",
        (ingredient_id, recipe_id, unit_id, amount),
        )?;
    Ok(())
}

// create new empty recipe with given name
fn create_recipe(conn: &Connection, name: &str) -> Result<i64> {
    match conn.execute(
        "INSERT OR IGNORE INTO recipes (name) VALUES (?1);",
        (name,),
        ) {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            Ok(id)
        },
        Err(e) => Err(e.into())
    }
}

fn get_recipe(conn: &Connection, recipe_id: i64) -> Result<Recipe> {
    let mut stmt = conn.prepare(
        //"SELECT ingredient_id, recipe_id, amount, unit_id, i.name, u.name FROM recipe_ingredient INNER JOIN ingredients i ON ingredient_id = i.id INNER JOIN units u ON unit_id = u.id WHERE recipe_id=?;")?;
        "SELECT ingredient_id, recipe_id, amount, unit_id, i.name, u.name, r.name FROM recipe_ingredient INNER JOIN ingredients i ON ingredient_id = i.id INNER JOIN units u ON unit_id = u.id INNER JOIN recipes r ON recipe_id = ? WHERE recipe_id=?;")?;
    let mut rows = stmt.query(rusqlite::params![recipe_id, recipe_id])?;

    let mut ingredients: Vec<(Ingredient, Amount, unit::Unit)> = Vec::new();
    let mut recipe_name = "".to_string();
    while let Some(row) = rows.next()? {
        recipe_name = row.get(6)?;
        let ingredient = Ingredient {
            id: row.get(0)?,
            name: row.get(4)?,
        };
        let unit = unit::Unit {
            id: row.get(3)?,
            name: row.get(5)?
        };
        let amount = Amount(row.get(2)?);
        ingredients.push((ingredient, amount, unit));
    }

    Ok(Recipe {
        id: recipe_id,
        name: recipe_name,
        ingredients: ingredients
    })

}
*/


// get recipes (-> Vec<Recipe>)
// get recipes by tag (tag_id: u64 -> Vec<Recipe>)
// get recipes by ingredient (ingredient_id: u64 -> Vec<Recipe>)
// get recipes by name (contains) (tag_id: u64 -> Vec<Recipe>)
// create recipe (name)
// add to recipe (ingredient, amount, )
// add tag to recipe (tag)
// FUTURE: get recipe nutrition (recipe_id)
//
//
// recipe_tags table
// recipe_ingredients amount? table



// get ingredients (Vec<Ingredient>)
// create ingredient (name)


/*
Tags menu
- view/create/delete tags

Ingredients menu
- view/create/delete ingredients 

Recipe menu
- view/create/delete recipes
- search recipes by TAG or NAME or INGREDIENT 
- Drag and drop ingredients onto selected menu
- Drag and drop tags onto selected menu

MealPlan menu
- view/create/delete meal plans
 */


mod unit;
mod tag;
mod ingredient;


fn main() -> Result<()> {
    //let conn = Connection::open(DATABASE_FILENAME)?;
    let conn = Connection::open_in_memory()?;

    unit::initialize(&conn)?;
    tag::initialize(&conn)?;
    ingredient::initialize(&conn)?;
   
    /*
    create_meal_plans_table(&conn)?;
    create_meal_plan_recipe_table(&conn)?;
    create_recipes_table(&conn)?;
    create_recipe_ingredient_table(&conn)?;

    let recipe = get_recipe(&conn, 1)?; 
    println!("{:#?}", recipe);
    */

    Ok(())
}



