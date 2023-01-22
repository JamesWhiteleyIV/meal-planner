use anyhow::Result;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};


// TODO
use crate::unit::Unit;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrition {
    pub unit: Unit,
    pub amount: f64,
    pub protein: f64,
    pub carbs: f64,
    pub fat: f64,
    pub soluble_fiber: f64,
    pub insoluble_fiber: f64,
}


const DEFAULT_INGREDIENTS: &'static [&str] = &[
    "instant pot",
    "poultry",
    "beef",
    "fish",
    "legumes",
    "rice",
    "grains",
    "noodles",
    "sauce",
    "smoothie",
    "oven",
];



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String, // e.g. carrot, potato
    pub nutrition: Option<Nutrition>
}


// initialize table and create defaults
pub fn initialize(conn: &Connection) -> Result<()> {
    create_table(&conn)?;
    for ingredient in DEFAULT_INGREDIENTS {
        create(&conn, ingredient)?;
    }
    Ok(())
}


pub fn create_table(conn: &Connection) -> Result<()> {
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

// create new ingredient
pub fn create(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO ingredients (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}


// get all ingredients from database
pub fn get(conn: &Connection) -> Result<Vec<Ingredient>> {
    let mut stmt = conn.prepare("SELECT id, name FROM ingredients")?;
    let rows = stmt.query_map([], |row| {
        Ok(Ingredient {
            id: row.get(0)?,
            name: row.get(1)?,
            nutrition: None
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}


// delete ingredient with given id from database
pub fn delete(conn: &Connection, ingredient_id: i64) -> Result<()> {
    conn.execute("DELETE FROM ingredients WHERE id=(?1);", (ingredient_id,),)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rusqlite::Connection;
    use crate::ingredient;

    #[test]
    fn test_initialize() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        ingredient::initialize(&conn)?;

        let ingredients = ingredient::get(&conn)?;
        assert_eq!(ingredients.len(), ingredient::DEFAULT_INGREDIENTS.len());
        Ok(())
    }


    #[test]
    fn test_create() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        ingredient::create_table(&conn)?;
        ingredient::create(&conn, "crazy beef")?;

        let ingredients= ingredient::get(&conn)?;
        assert_eq!(ingredients.len(), 1);

        let ingredient = ingredients[0].clone();
        assert_eq!(ingredient.name, "crazy beef".to_string());

        Ok(())
    }


    #[test]
    fn test_delete() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        ingredient::initialize(&conn)?;

        let ingredients = ingredient::get(&conn)?;
        let ingredient_to_delete = ingredients[0].clone();
        ingredient::delete(&conn, ingredient_to_delete.id);

        let ingredients = ingredient::get(&conn)?;
        assert_eq!(ingredients.len(), ingredient::DEFAULT_INGREDIENTS.len() - 1);

        for ingredient in ingredients {
            assert_ne!(ingredient_to_delete.id, ingredient.id);
            assert_ne!(ingredient_to_delete.name, ingredient.name);
        }

        Ok(())
    }

}
    



