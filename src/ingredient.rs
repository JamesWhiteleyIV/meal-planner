use anyhow::Result;
use rusqlite::{params, Connection};
use crate::DATABASE_FILENAME;
use food_item::FoodItem;


#[derive(Debug, Clone)]
pub struct Ingredient {
    pub id: u64,
    pub food_item: FoodItem,
    pub amount: f64,  // e.g. 4, .5
    pub unit: Unit    // e.g. ml, fl oz, lbs
}


#[derive(Debug, Clone)]
pub struct _Ingredient{
    pub id: u64,
    pub amount: f64,  // e.g. 4, .5
}

impl Ingredient {
    pub fn create_database_table(conn: &Connection) -> Result<()> {
        // TODO figure out relationship
        conn.execute(
        "CREATE TABLE IF NOT EXISTS ingredients (
            id INTEGER NOT NULL AUTO INCREMENT PRIMARY KEY,
            food_item_id INTEGER,
            amount FLOAT,
            unit_id INTEGER
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

    pub fn from_database(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, food_item_id, amount, unit_id FROM ingredients")?;
        let rows = stmt.query_map([], |row| {
            Ok(_Ingredient {
                // TODO
                id: row.get(0)?,
                amount: 0.0
            })
        })?;

        let recipes = rows.into_iter().flatten().collect();
        Ok(recipes)
    }

}

