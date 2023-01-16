// https://docs.rs/rusqlite/0.28.0/rusqlite/struct.Statement.html

use anyhow::Result;
use rusqlite::{params, Connection};


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

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: i64,
    pub name: String
}



fn create_units_table(conn: &Connection) -> Result<()> {
    println!("creating units table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS units (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL, 
        UNIQUE (name)
    )",
    (), 
    )?;

    Ok(())
}

fn create_unit(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO units (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}

fn get_units(conn: &Connection) -> Result<Vec<Unit>> {
    let mut stmt = conn.prepare("SELECT id, name FROM units")?;
    let rows = stmt.query_map([], |row| {
        Ok(Unit {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}

fn delete_unit(conn: &Connection, unit_id: i64) -> Result<()> {
    conn.execute("DELETE FROM units WHERE id=(?1);", (unit_id,),)?;
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String
}

fn create_tags_table(conn: &Connection) -> Result<()> {
    println!("creating tags table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL, 
        UNIQUE (name)
    )",
    (), 
    )?;

    Ok(())
}

// creates Tag in database if name is unique
fn create_tag(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO tags (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}

fn get_tags(conn: &Connection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name FROM tags")?;
    let rows = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}

fn delete_tag(conn: &Connection, tag_id: i64) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE id=(?1);", (tag_id,),)?;
    Ok(())
}


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

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount(f64);

 
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String, // e.g. carrot, potato
    pub ingredients: Vec<(Ingredient, Amount, Unit)>
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

    let mut ingredients: Vec<(Ingredient, Amount, Unit)> = Vec::new();
    let mut recipe_name = "".to_string();
    while let Some(row) = rows.next()? {
        recipe_name = row.get(6)?;
        let ingredient = Ingredient {
            id: row.get(0)?,
            name: row.get(4)?,
        };
        let unit = Unit {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String, // e.g. carrot, potato
    // TODO: pub nutrition: Nutrition
}

fn create_ingredients_table(conn: &Connection) -> Result<()> {
    println!("creating ingredients table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        UNIQUE(name)
    )",
    (), 
    )?;

    Ok(())
}


fn create_ingredient(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO ingredients (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}

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


fn main() -> Result<()> {
    let conn = Connection::open(DATABASE_FILENAME)?;
   
    create_units_table(&conn)?;
    create_tags_table(&conn)?;
    create_ingredients_table(&conn)?;
    create_meal_plans_table(&conn)?;
    create_meal_plan_recipe_table(&conn)?;
    create_recipes_table(&conn)?;
    create_recipe_ingredient_table(&conn)?;

    create_unit(&conn, "oz")?;
    create_unit(&conn, "lb")?;
    create_unit(&conn, "g")?;
    create_unit(&conn, "fl oz")?;
    create_unit(&conn, "tsp")?;
    create_unit(&conn, "Tbsp")?;
    create_unit(&conn, "cup")?;
    create_unit(&conn, "ml")?;
 
    create_tag(&conn, "instant pot")?;
    create_tag(&conn, "oven")?;
    create_tag(&conn, "pan")?;
    create_tag(&conn, "pot")?;

    create_ingredient(&conn, "apple")?;
    create_ingredient(&conn, "carrot")?;
    create_ingredient(&conn, "black beans")?;
    create_ingredient(&conn, "top sirloin steak")?;
    create_ingredient(&conn, "butter")?;
    create_ingredient(&conn, "rosemary")?;

    create_recipe(&conn, "top sirloin")?;
    add_ingredient_to_recipe(&conn, 1, 1, 3, 10.0)?;
    add_ingredient_to_recipe(&conn, 3, 1, 5, 2.3)?;
    add_ingredient_to_recipe(&conn, 4, 1, 7, 0.5)?;


    for t in get_units(&conn)? {
        println!("{:#?}", t);
    };
 
    /*
    for t in get_tags(&conn)? {
        println!("{:#?}", t);
    };
    */

    let recipe = get_recipe(&conn, 1)?; 
    println!("{:#?}", recipe);

    //println!("{:#?}", recipe_id);


    /*
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        data JSON NOT NULL 
    )",
    (), // empty list of parameters.
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        data JSON NOT NULL 
    )",
    (), // empty list of parameters.
    )?;


    let test = Test {
        name: vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()]
    };

    conn.execute(
        "INSERT OR IGNORE INTO test (data) VALUES (?1);",
        (serde_json::to_string(&test)?,),
    )?;

    //Unit::create_database_table(&conn)?;
    //FoodItem::create_database_table(&conn)?;
    //Recipe::create_database_table(&conn)?;
    */

    Ok(())
}



