use anyhow::Result;
use rusqlite::{params, Connection};
use crate::DATABASE_FILENAME;


const DEFAULT_FOOD_ITEMS: &'static [&str] = &[
    "avocado",
    "black beans",
    "carrot",
    "pinto beans",
    "tortilla",
];


// TODO: nutrition info
#[derive(Debug, Clone)]
pub struct FoodItem {
    pub id: u64,
    pub name: String, // e.g. carrot, potato
}


impl FoodItem {
    pub fn create_database_table(conn: &Connection) -> Result<()> {
        println!("creating food_items table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS food_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL, 
            UNIQUE (name)
        )",
        (), // empty list of parameters.
        )?;

        for i in DEFAULT_FOOD_ITEMS  {
            conn.execute(
                "INSERT OR IGNORE INTO food_items (name) VALUES (?1);",
                (i,),
                )?;
        }

        Ok(())

    }


    pub fn from_database(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, name FROM food_items")?;
        let rows = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?
            })
        })?;

        let food_items = rows.into_iter().flatten().collect();
        Ok(food_items)
    }


}



