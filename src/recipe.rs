use anyhow::Result;
use rusqlite::{params, Connection};


#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub notes: String,
    //pub tags: Vec<String>
}


impl Recipe {
    pub fn create_database_table(conn: &Connection) -> Result<()> {
        println!("creating recipes table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS recipes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                ingredients JSON,
                instructions JSON,
                notes TEXT,
            )",
            (), // empty list of parameters.
        )?;

        Ok(())
    }

    pub fn from_database(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, name, ingredients, instructions, notes, tags FROM units")?;
        let rows = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?,
                // TODO
                ingredients: vec![],
                instructions: vec![],
                notes: "".to_string(),
                tags: vec![]
            })
        })?;

        let recipes = rows.into_iter().flatten().collect();
        Ok(recipes)
    }

    pub fn multiply(&mut self, multiple: f64) -> Self {
        let mut recipe = self.clone();
        for ingredient in recipe.ingredients.iter_mut() {
            ingredient.amount *= multiple
        }
        recipe
    }

}

