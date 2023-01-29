use anyhow::Result;
use rusqlite::{Connection, ValueRef, FromSqlResult, FromSql};
use serde::{Serialize, Deserialize};
//use crate::{tag::Tag, ingredient::Ingredient};


// recipe
//    recipes_ingredients
//    recipes_tags
//    Recipe::add_tag
//    Recipe::remove_tag
//    Recipe::add_ingredient
//    Recipe::remove_ingredient
// ingredient
//    Ingredient::update_nutrition
// tag
// unit


const DEFAULT_RECIPES: &'static [&str] = &[
    "tri-tip",
];


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub instructions: Vec<String>,
    pub notes: Vec<String>,
    //pub ingredients: Vec<Ingredient>,
    //pub tags: Vec<Tag>,
}


// initialize tables and create default recipes
pub fn initialize(conn: &Connection) -> Result<()> {
    create_table(&conn)?;
    for recipe in DEFAULT_RECIPES {
        create(&conn, recipe)?;
    }
    Ok(())
}


pub fn create_table(conn: &Connection) -> Result<()> {
    println!("creating recipes table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            instructions TEXT,
            notes TEXT,
        )",
        (), // empty list of parameters.
    )?;

    println!("creating recipes_tags table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
        )",
        (), // empty list of parameters.
    )?;

    println!("creating recipes_ingredients table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes_ingredients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            ingredient_id INTEGER NOT NULL,
        )",
        (), // empty list of parameters.
    )?;
 
    Ok(())
}


pub fn create(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO recipes (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}


pub fn update(conn: &mut Connection, recipe_id: i64, name: Option<String>, instructions: Option<Vec<String>>, notes: Option<Vec<String>>) -> Result<()> {
    let tx = conn.transaction()?;

    if let Some(name) = name {
        tx.execute("UPDATE recipes SET name = (?1) WHERE id = (?2)", &[&name, &recipe_id.to_string()])?;
    }

    if let Some(instructions) = instructions {
        let instructions = serde_json::to_string(&instructions)?;
        tx.execute("UPDATE recipes SET instructions = (?1) WHERE id = (?2)", &[&instructions, &recipe_id.to_string()])?;
    }

     if let Some(notes) = notes {
        let notes = serde_json::to_string(&notes)?;
        tx.execute("UPDATE recipes SET notes = (?1) WHERE id = (?2)", &[&notes, &recipe_id.to_string()])?;
    }
 
    tx.commit()?;
    Ok(())
}


pub fn get(conn: &Connection) -> Result<Vec<Recipe>> {
    let mut stmt = conn.prepare("SELECT id, name, instructions, notes")?;
    let rows = stmt.query_map([], |row| {
        Ok(Recipe {
            id: row.get(0)?,
            name: row.get(1)?,
            instructions: row.get(2)?,
            notes: row.get(3)?,
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}


pub fn delete(conn: &mut Connection, recipe_id: i64) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM recipes WHERE id=(?1);", &[&recipe_id.to_string()])?;
    tx.execute("DELETE FROM recipes_tags WHERE recipe_id=(?1);", &[&recipe_id.to_string()])?;
    tx.execute("DELETE FROM recipes_ingredients WHERE recipe_id=(?1);", &[&recipe_id.to_string()])?;
    tx.commit()?;

    Ok(())
}


pub fn add_tag(conn: &Connection, recipe_id: i64, tag_id: i64) -> Result<()> {
    Ok(())
}

pub fn remove_tag(conn: &Connection, recipe_id: i64, tag_id: i64) -> Result<()> {
    Ok(())
}

pub fn add_ingredient(conn: &Connection, recipe_id: i64, ingredient_id: i64) -> Result<()> {
    Ok(())
}


pub fn remove_ingredient(conn: &Connection, recipe_id: i64, ingredient_id: i64) -> Result<()> {
    Ok(())
}


/* TODO
pub fn multiply(&mut self, multiple: f64) -> Self {
    let mut recipe = self.clone();
    for ingredient in recipe.ingredients.iter_mut() {
        ingredient.amount *= multiple
    }
    recipe
}
*/


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rusqlite::Connection;
    use crate::recipe;

    #[test]
    fn test_initialize() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        recipe::initialize(&conn)?;

        let recipes = recipe::get(&conn)?;
        assert_eq!(recipes.len(), recipe::DEFAULT_RECIPES.len());
        Ok(())
    }


    #[test]
    fn test_create() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        recipe::create_table(&conn)?;
        recipe::create(&conn, "crazy beef")?;

        let recipes = recipe::get(&conn)?;
        assert_eq!(recipes.len(), 1);

        let recipe = recipes[0].clone();
        assert_eq!(recipe.name, "crazy beef".to_string());

        Ok(())
    }


    #[test]
    fn test_delete() -> Result<()> {
        let mut conn = Connection::open_in_memory()?;
        recipe::initialize(&conn)?;

        let recipes = recipe::get(&conn)?;
        let recipe_to_delete = recipes[0].clone();
        recipe::delete(&mut conn, recipe_to_delete.id);

        let recipes = recipe::get(&conn)?;
        assert_eq!(recipes.len(), recipe::DEFAULT_RECIPES.len() - 1);

        for recipe in recipes {
            assert_ne!(recipe_to_delete.id, recipe.id);
            assert_ne!(recipe_to_delete.name, recipe.name);
        }

        Ok(())
    }

}
    


