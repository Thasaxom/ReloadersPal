mod sql;
use sql::Database;

fn main() {
        
    let database = Database::new("./loaddata.db");

    let pairs: Vec<(i32, String)> = database.get_id_pairs("casing", "name");

    for (id, name) in pairs.iter() {
        println!("{} {}", id, name);
    }
    
}
