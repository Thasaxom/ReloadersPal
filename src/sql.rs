use rusqlite::{Connection, Result, OpenFlags, params};
use rusqlite::types::ToSql;
use std::process;
use reloaders_pal::{Casing, Projectile, Powder, Load, BallisticTest};

// wrapper to hold the database connection and do work with it
// makes the function parameters simpler by removing the need to pass the 
// connection to them for each function
pub struct Database {

    conn: Connection,
    query: String,

}

impl Database {

    pub fn new(path: &str) -> Database {
        Database {
            conn: Database::open_connection(path),
            query: String::new(),
        }
    }

    fn open_connection(path: &str) -> Connection {

        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE);

        match conn {
            Result::Ok(opened_conn) => {
                println!("database found");
                opened_conn
            }
            Result::Err(_) => {
                println!("database not found");
                process::exit(1);
            }
        }
    }

    pub fn select(&mut self, column: &str) -> &mut Self {

        let format = format!("SELECT {}", column);
        self.query.push_str(&format);

        self

    }

    pub fn from(&mut self, table: &str) -> &mut Self {

        let format = format!(" FROM {}", table);
        self.query.push_str(&format);

        self

    }

    pub fn from_where(&mut self, table: &str) -> &mut Self {

        let format = format!(" FROM {} WHERE", table);
        self.query.push_str(&format);

        self

    }

    pub fn field(&mut self, field: &str) -> &mut Self {

        let format = format!(" {}", field);
        self.query.push_str(&format);

        self

    }

    pub fn equals(&mut self, value: &str) -> &mut Self {

        let format = format!(" = '{}'", value);
        self.query.push_str(&format);

        self

    }

    pub fn get_query(&self) -> &str {
    
        &self.query

    }
}

pub enum SqlValue<T: ToSql> {

    Text(T),
    Num(T),
    Wildcard,
    None,

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_builders() {

        let mut database = Database::new("./loaddata.db");

        database.select("*").from_where("casing").field("name").equals(".357 Magnum");
        
        assert_eq!(database.get_query(), "SELECT * FROM casing WHERE name = '.357 Magnum'");

    }
}
