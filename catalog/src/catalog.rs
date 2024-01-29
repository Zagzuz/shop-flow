use crate::catalog_proto::Item as ProtoItem;
use compact_str::CompactString;
use eyre::eyre;
use rusqlite::Connection;
use std::{path::Path, sync::Mutex};

pub struct Catalog {
    db_conn: Mutex<Connection>,
    db_table: CompactString,
}

impl Catalog {
    pub fn new(db_path: &Path, db_table: &str) -> Self {
        Self {
            db_conn: Mutex::new(Connection::open(db_path).expect("db connection failed")),
            db_table: db_table.into(),
        }
    }

    pub fn list_items(&self) -> eyre::Result<Vec<Item>> {
        let conn = self.db_conn.lock().map_err(|err| eyre!(err.to_string()))?;
        let mut stmt = conn.prepare(&format!(
            "SELECT title, price, item_count FROM {};",
            self.db_table
        ))?;
        let items = stmt
            .query_map([], |row| {
                Ok(Item {
                    title: row.get::<_, String>(0)?.into(),
                    price: row.get(1)?,
                    count: row.get(2)?,
                })
            })?
            .into_iter()
            .collect::<Result<Vec<Item>, _>>()?;
        Ok(items)
    }

    pub fn find_item(&self, query: &str) -> eyre::Result<Vec<Item>> {
        let conn = self.db_conn.lock().map_err(|err| eyre!(err.to_string()))?;
        let mut stmt = conn.prepare(&format!(
            "SELECT title, price, item_count FROM {} WHERE title LIKE '%{}%';",
            self.db_table, query
        ))?;
        let items = stmt
            .query_map([], |row| {
                Ok(Item {
                    title: row.get::<_, String>(0)?.into(),
                    price: row.get(1)?,
                    count: row.get(2)?,
                })
            })?
            .into_iter()
            .collect::<Result<Vec<Item>, _>>()?;
        Ok(items)
    }
}

#[derive(Clone)]
pub struct Item {
    pub title: CompactString,
    pub price: f32,
    pub count: u32,
}

#[allow(clippy::from_over_into)]
impl Into<ProtoItem> for Item {
    fn into(self) -> ProtoItem {
        ProtoItem {
            title: self.title.into(),
            price: self.price,
            count: self.count,
        }
    }
}
