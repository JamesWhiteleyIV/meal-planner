use anyhow::Result;
use rusqlite::{params, Connection};
use crate::DATABASE_FILENAME;


const DEFAULT_UNITS: &'static [&str] = &[
    "oz",
    "lb",
    "g",
    "fl oz",
    "tsp",
    "Tbsp",
    "cup",
    "ml"
];

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: u64,
    pub name: String, // e.g. ml, fl oz, lbs
}


impl Unit {
    pub fn create_database_table(conn: &Connection) -> Result<()> {
        println!("creating units table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS units (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                UNIQUE (name)
            )",
            (), 
        )?;

        for unit in DEFAULT_UNITS {
            conn.execute(
                "INSERT OR IGNORE INTO units (name) VALUES (?1);",
                (unit,),
            )?;
        }

        Ok(())
    }
}


impl Unit {
    pub fn from_database(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, name FROM units")?;
        let rows = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?
            })
        })?;

        let units = rows.into_iter().flatten().collect();
        Ok(units)
    }

}

