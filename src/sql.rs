use rusqlite::{Connection, Result, OpenFlags, params};
use rusqlite::types::FromSql;
use std::process;
use reloaders_pal::Casing;

struct Database {

    conn: Connection,

}

impl Database {

    pub fn new(path: &str) -> Database {
        Database {
            conn: Database::open_connection(path),
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

    pub fn get_id_pairs<T: FromSql>(&self, table: &str, column: &str) -> Vec<(i32, T)> {
        
        let mut query = String::from("SELECT * FROM ");
        query.push_str(table);
        let mut stmt = self.conn.prepare(&query).unwrap();
        let rows = stmt.query_map(params![], |row| {
            Ok((row.get(0)?, row.get(column)?))
        }).unwrap();

        let mut pairs = Vec::new();
        for pair in rows {
           pairs.push(pair.unwrap());
        }

        pairs

    }
        
    pub fn get_casing(&self, id: i32) -> Casing {
        
        let new_casing = self.conn.query_row("SELECT * FROM casing WHERE casing_id = ?1", params![id], |row| {
            Ok(Casing {
                casing_id: row.get("casing_id")?,
                name: row.get("name")?,
                primer_size: row.get("primer_size")?,
                case_type: row.get("case_type")?,
                max_psi: row.get("max_psi")?,
                max_cup: row.get("max_cup")?,
            })
        }).unwrap();

        new_casing
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_id_pairs() {

        let database = Database::new("./loaddata.db");
        let pairs = database.get_id_pairs("casing", "name");

        assert_eq!(vec![(1, ".357 Magnum".to_string()),
                        (2, "5.56x45mm NATO".to_string()),
                        (3, ".45 ACP".to_string()),
                        (4, "9x19mm Parabellum".to_string()),
                        (5, ".45-70 Government".to_string())], pairs);

        let pairs = database.get_id_pairs("projectile", "diameter");

        assert_eq!(vec![(1, 0.356),
                        (2, 0.355),
                        (3, 0.458),
                        (5, 0.357)], pairs);
    }

    #[test]
    fn test_get_functions() {

        let database = Database::new("./loaddata.db");
        let test_casing = database.get_casing(1);
        //let test_projectile = database.get_projectile(1);
        //let test_powder = database.get_powder(1);
        //let test_ballistic_test = database.get_ballistic_test(1);
        //let test_load = database.get_load(1);

        assert!(test_casing.casing_id == 1); 
        assert!(test_casing.name == ".357 Magnum".to_string());
        assert!(test_casing.primer_size == "SPM".to_string());
        assert!(test_casing.case_type == "Rimmed, straight".to_string());
        assert!(test_casing.max_psi == 35000.0);
        assert!(test_casing.max_cup == 45000.0);



    }
}
