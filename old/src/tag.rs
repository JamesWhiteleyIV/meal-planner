use anyhow::Result;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};


const DEFAULT_TAGS: &'static [&str] = &[
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
pub struct Tag {
    pub id: i64,
    pub name: String
}


// initialize table and create defaults
pub fn initialize(conn: &Connection) -> Result<()> {
    create_table(&conn)?;
    for tag in DEFAULT_TAGS {
        create(&conn, tag)?;
    }
        
    Ok(())
} 


// create table in database
pub fn create_table(conn: &Connection) -> Result<()> {
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


// create new tag (ignores if name already exists)
pub fn create(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO tags (name) VALUES (?1);",
        (name,),
        )?;
    Ok(())
}


// get all tags from database
pub fn get(conn: &Connection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name FROM tags")?;
    let rows = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    Ok(rows.into_iter().flatten().collect())
}


// delete tag with given id from database
pub fn delete(conn: &Connection, tag_id: i64) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE id=(?1);", (tag_id,),)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rusqlite::Connection;
    use crate::tag;

    #[test]
    fn test_initialize() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        tag::initialize(&conn)?;

        let tags = tag::get(&conn)?;
        assert_eq!(tags.len(), tag::DEFAULT_TAGS.len());
        Ok(())
    }


    #[test]
    fn test_create() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        tag::create_table(&conn)?;
        tag::create(&conn, "crazy beef")?;

        let tags= tag::get(&conn)?;
        assert_eq!(tags.len(), 1);

        let tag = tags[0].clone();
        assert_eq!(tag.name, "crazy beef".to_string());

        Ok(())
    }


    #[test]
    fn test_delete() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        tag::initialize(&conn)?;

        let tags = tag::get(&conn)?;
        let tag_to_delete = tags[0].clone();
        tag::delete(&conn, tag_to_delete.id);

        let tags = tag::get(&conn)?;
        assert_eq!(tags.len(), tag::DEFAULT_TAGS.len() - 1);

        for tag in tags {
            assert_ne!(tag_to_delete.id, tag.id);
            assert_ne!(tag_to_delete.name, tag.name);
        }

        Ok(())
    }

}
    



