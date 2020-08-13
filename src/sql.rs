use rusqlite::{Connection, Result, OpenFlags, params};
use rusqlite::types::ToSql;
use std::process;
use reloaders_pal::{Casing, Projectile, Powder, Load, BallisticTest};

// holds the database connection and current query
// builder functions let you build an sql query from start to finish
pub struct Database {

    conn: Connection,
    query: Option<Query>,

}

impl Database {

    pub fn new(path: &str) -> Database {
        Database {
            conn: Database::open_connection(path),
            query: Option::None,
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

    // Starts a new SELECT query
    // Panics if another query is already being built or used
    pub fn select(&mut self) -> &mut Self {

        match self.query {

            Some(_) => panic!("unused query cannot be overwritten"),
            None => self.query = Option::Some(Query::new(SqlCommand::Select)),

        };

        self

    }

    // Adds new parameters to the query
    // Panics if there is no base sql command chosen
    pub fn from(&mut self, table: &str) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.add_table(table),

        }

        self

    }
    
    pub fn column(&mut self, column: &str) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.add_column(column),

        }

        self

    }

    pub fn field(&mut self, field: &str) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.add_field(field),

        }

        self

    }

    pub fn value(&mut self, value: SqlVal) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.add_value(value),

        }

        self

    }

    pub fn op(&mut self, op: SqlOp) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.add_operator(op),

        }

        self

    }

    pub fn get_query(&mut self) -> String {
    
        match &mut self.query {

            None => panic!("can't build empty query"),
            Some(query) => query.build(),

        }

    }
    
}

// Holds the query, then builds and executes it
struct Query {

   command: SqlCommand,
   tables: Vec<String>,
   columns: Vec<String>,
   fields: Vec<String>,
   values: Vec<SqlVal>,
   operators: Vec<SqlOp>,

}

impl Query {

    pub fn new(given_command: SqlCommand) -> Query {

        Query {
            command: given_command,
            tables: Vec::new(),
            columns: Vec::new(),
            fields: Vec::new(),
            values: Vec::new(),
            operators: Vec::new(),
        }

    }

    pub fn add_table(&mut self, table: &str) {

        self.tables.push(table.to_string());

    }

    pub fn add_column(&mut self, column: &str) {

        self.columns.push(column.to_string());

    }

    pub fn add_field(&mut self, field: &str) {

        self.fields.push(field.to_string());

    }

    pub fn add_value(&mut self, value: SqlVal) {

        self.values.push(value);

    }

    pub fn add_operator(&mut self, operator: SqlOp) {

        self.operators.push(operator);

    }

    pub fn build(&mut self) -> String {

        "placeholder".to_string()

    }

}

// Ensures values are of the correct type 
pub enum SqlVal {

    Text(String),
    Num(f64),
    None,

}

pub enum SqlOp {

    And,
    Or,
    Not,
    Equals,

}

// Only used by the Database struct for the main command
enum SqlCommand {

    Select,
    Insert,
    Update,
    Delete,

}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_builders() {

        let mut database = Database::new("./loaddata.db");

        database
            .select()
            .column("*")
            .from("casing")
            .field("name")
            .op(SqlOp::Equals)
            .value(SqlVal::Text(".357 Magnum".to_string()));
        
        assert_eq!(database.get_query(), "SELECT * FROM casing WHERE name = '.357 Magnum'");

    }
    
}
