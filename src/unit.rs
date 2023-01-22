use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Serialize, Deserialize};


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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: i64,
    pub name: String
}


// initialize table and create defaults
pub fn initialize(conn: &Connection) -> Result<()> {
    create_table(&conn)?;
    for unit in DEFAULT_UNITS {
        create(&conn, unit)?;
    }
        
    Ok(())
} 


// create table in database
pub fn create_table(conn: &Connection) -> Result<()> {
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


// create new unit (ignores if name already exists)
pub fn create(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO units (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}


// get all units from database
pub fn get(conn: &Connection) -> Result<Vec<Unit>> {
    let mut stmt = conn.prepare("SELECT id, name FROM units")?;
    let rows = stmt.query_map([], |row| {
        Ok(Unit {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}


// delete unit with given id from database
pub fn delete(conn: &Connection, unit_id: i64) -> Result<()> {
    conn.execute("DELETE FROM units WHERE id=(?1);", (unit_id,),)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rusqlite::{params, Connection};
    use crate::unit;

    #[test]
    fn test_initialize() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        unit::initialize(&conn)?;

        let units = unit::get(&conn)?;
        assert_eq!(units.len(), unit::DEFAULT_UNITS.len());
        Ok(())
    }


    #[test]
    fn test_create() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        unit::create_table(&conn)?;
        unit::create(&conn, "ml")?;

        let units = unit::get(&conn)?;
        assert_eq!(units.len(), 1);

        let unit = units[0].clone();
        assert_eq!(unit.name, "ml".to_string());

        Ok(())
    }


    #[test]
    fn test_delete() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        unit::initialize(&conn)?;

        let units = unit::get(&conn)?;
        let unit_to_delete = units[0].clone();
        unit::delete(&conn, unit_to_delete.id);

        let units = unit::get(&conn)?;
        assert_eq!(units.len(), unit::DEFAULT_UNITS.len() - 1);

        for unit in units {
            assert_ne!(unit_to_delete.id, unit.id);
            assert_ne!(unit_to_delete.name, unit.name);
        }

        Ok(())
    }

}
    



