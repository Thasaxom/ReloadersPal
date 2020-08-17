use rusqlite::{Connection, Result, OpenFlags, params};
use rusqlite::types::ToSql;
use std::process;
use reloaders_pal::{Casing, Projectile, Powder, Load, BallisticTest};

#[macro_export]
macro_rules! fields {
    
    ($database:ident, $( $val:expr ),*) => {
        
        $database
        $(
            .field($val)
        )*
    }

}

#[macro_export]
macro_rules! values {
    
    ($database:ident, $( $val:expr ),*) => {
        
        $database
        .start()
        $(
            .value($val)
        )*
        .end()
    }

}

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

    pub fn insert(&mut self) -> &mut Self {

        match self.query {

            Some(_) => panic!("unused query cannot be overwritten"),
            None => self.query = Option::Some(Query::new(SqlCommand::Insert)),

        };

        self

    }

    // Adds new parameters to the query
    // Panics if there is no base sql command chosen
    pub fn table(&mut self, table: &str) -> &mut Self {

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

    pub fn start(&mut self) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.start_values(),

        }

        self

    }

    pub fn end(&mut self) -> &mut Self {

        match &mut self.query {

            None => panic!("need sql command first"),
            Some(query) => query.end_values(),

        }

        self

    }
    pub fn condition(&mut self, field: &str, op: SqlOp, val: SqlVal) -> &mut Self {

        match &mut self.query {
            None => panic!("need sql command first"),
            Some(query) => query.add_condition(field, op, val),

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

    pub fn reset_query(&mut self) {

        self.query = Option::None;

    }
    
}

// Holds the query, then builds and executes it
struct Query {

   command: SqlCommand,
   tables: Vec<String>,
   columns: Vec<String>,
   fields: Vec<String>,
   values: Vec<Vec<SqlVal>>,
   values_count: usize,
   conditions: Vec<(String, SqlOp, SqlVal)>,
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
            values_count: 0,
            conditions: Vec::new(),
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

        self.values[self.values_count].push(value);

    }

    pub fn add_condition(&mut self, field: &str, op: SqlOp, value: SqlVal) {

        self.conditions.push((field.to_string(), op, value));

    }

    pub fn add_operator(&mut self, operator: SqlOp) {

        self.operators.push(operator);

    }

    pub fn start_values(&mut self) {

        self.values.push(Vec::new());

    }

    pub fn end_values(&mut self) {

        self.values_count += 1;

    }

    // checks the command type and call the appropriate function
    pub fn build(&mut self) -> String {
        match self.command {

            SqlCommand::Select => self.build_select(),
            SqlCommand::Insert => self.build_insert(),
            _ => panic!("not implemented"),

        }
    }

    // takes a reference to a vector of strings and returns the contents as 
    // a comma seperated list. if the vector has 1 elements, just returns that element
    // panics if given an empty list
    fn comma_seperated_list(list_items: &Vec<String>) -> String {

        let len = list_items.len();
        let mut list = String::new();

        if len > 1 {

            list.push_str(list_items[0].as_str());

            for item in list_items[1..].iter() {

                list.push_str(",");
                list.push_str(item.as_str());

            }
        }
        else if len == 1 { list.push_str(list_items[0].as_str()); }
        else { panic!("no list_items given"); }

        list

    }

    fn comma_seperated_values(list_items: &Vec<SqlVal>) -> String {

        let len = list_items.len();
        let mut list = String::new();
        list.push_str("(");

        if len > 1 {

            match &list_items[0] {
                SqlVal::Text(value) => list.push_str(format!("'{}'", value).as_str()),
                SqlVal::Int(value) => list.push_str(value.to_string().as_str()),
                SqlVal::Real(value) => list.push_str(value.to_string().as_str()),
            }

            for item in list_items[1..].iter() {

                list.push_str(",");
                match item {
                    SqlVal::Text(value) => list.push_str(format!("'{}'", value).as_str()),
                    SqlVal::Int(value) => list.push_str(value.to_string().as_str()),
                    SqlVal::Real(value) => list.push_str(value.to_string().as_str()),
                }

            }
        }
        else if len == 1 { 
            match &list_items[0] {
                SqlVal::Text(value) => list.push_str(format!("'{}'", value).as_str()),
                SqlVal::Int(value) => list.push_str(value.to_string().as_str()),
                SqlVal::Real(value) => list.push_str(value.to_string().as_str()),
            }
        }
        else { panic!("no list_items given"); }
        list.push_str(")");

        list

    }

    // builder for select queries
    fn build_select(&mut self) -> String {

        let mut built_query = String::new();

        // push the columns as a comma seperated list
        built_query.push_str("SELECT ");
        built_query.push_str(Query::comma_seperated_list(&self.columns).as_str());

        // push the tables as a comma seperated list
        built_query.push_str(" FROM ");
        built_query.push_str(Query::comma_seperated_list(&self.tables).as_str());

        // if there are no conditions, return the query as it is now
        if self.conditions.len() == 0 {
            return built_query;
        }

        built_query.push_str(" WHERE ");

        // get an iterator over the logical operators between conditions
        let mut operators_iter = self.operators.iter();

        for (field, op, value) in self.conditions.iter() {

            //push the field as is
            built_query.push_str(field.as_str());

            // match the correct op and push the correct symbols
            match op {
                SqlOp::Equals => built_query.push_str("="),
                SqlOp::NotEquals => built_query.push_str("<>"),
                SqlOp::GT => built_query.push_str(">"),
                SqlOp::GTE => built_query.push_str(">="),
                SqlOp::LT => built_query.push_str("<"),
                SqlOp::LTE => built_query.push_str("<="),
                _ => panic!("invalid or unimplemented operator for select condition"),
            }

            // push a string with quotes around it and push a number as is
            match value {
                SqlVal::Text(value) => built_query.push_str(format!("'{}'", value).as_str()),
                SqlVal::Int(value) => built_query.push_str(value.to_string().as_str()),
                SqlVal::Real(value) => built_query.push_str(value.to_string().as_str()),
            }

            // if there is another logical operator, push the correct symbol to the query
            // otherwise do nothing
            if let Some(logical_op) = operators_iter.next() {

                match logical_op {
                    SqlOp::And => built_query.push_str(" AND "),
                    SqlOp::Or => built_query.push_str(" OR "),
                    _ => panic!("invalid or unimplemented operator for select condition"),
                }
            }
        }
        built_query
    }

    fn build_insert(&mut self) -> String {

        let mut built_query = String::new();

        if self.tables.len() != 1 {
            panic!("too many or too few tables for insert statement");
        }

        built_query.push_str("INSERT INTO ");
        built_query.push_str(self.tables[0].as_str());

        if self.fields.len() > 0 {
            built_query.push_str(format!(" ({})", Query::comma_seperated_list(&self.fields)).as_str());
        }

        built_query.push_str(" VALUES ");

        let mut value_sets: Vec<String> = Vec::new();

        for set in self.values.iter() {

            value_sets.push(Query::comma_seperated_values(&set));

        }

        built_query.push_str(Query::comma_seperated_list(&value_sets).as_str());

        built_query

    }
}

// Ensures values are of the correct type 
pub enum SqlVal {

    Text(String),
    Int(i32),
    Real(f64),

}

pub enum SqlOp {

    And,
    Or,
    Not,
    Equals,
    NotEquals,
    GT,
    LT,
    GTE,
    LTE,

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
    fn test_simple_select() {

        let mut database = Database::new("./loaddata.db");

        database
            .select()
            .column("*")
            .table("projectile");

        assert_eq!(database.get_query(), "SELECT * FROM projectile");
    }
    
    #[test]
    fn test_complex_select() {

        let mut database = Database::new("./loaddata.db");

        database
            .select()
            .column("name")
            .column("primer_size")
            .table("casing")
            .condition("name", SqlOp::Equals, SqlVal::Text(".357 Magnum".to_string()))
            .op(SqlOp::Or)
            .condition("casing_id", SqlOp::Equals, SqlVal::Int(1));
        
        assert_eq!(database.get_query(), "SELECT name,primer_size FROM casing WHERE name='.357 Magnum' OR casing_id=1");
    }

    #[test]
    fn test_insert() {

        let mut database = Database::new("./loaddata.db");

        database
            .insert()
            .table("casing");

        fields!(database,
            "casing_id",
            "name",
            "type",
            "max_psi");

        values!(database, 
                SqlVal::Int(6), 
                SqlVal::Text(".30-06".to_string()), 
                SqlVal::Text("Rimless, Straight bottleneck".to_string()),
                SqlVal::Real(25000.0));

        assert_eq!(database.get_query(), "INSERT INTO casing (casing_id,name,type,max_psi) VALUES (6,'.30-06','Rimless, Straight bottleneck',25000)");

    }

    #[test]
    #[should_panic]
    fn builder_panic() {

        let mut database = Database::new("./loaddata.db");

        database.column("panic time");

    }
    
}
