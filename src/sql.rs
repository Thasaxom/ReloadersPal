use rusqlite::{Connection, Result, OpenFlags, params};
use rusqlite::types::FromSql;
use std::process;
use reloaders_pal::{Casing, Projectile, Powder, Load, BallisticTest};

// wrapper to hold the database connection and do work with it
// makes the function parameters simpler by removing the need to pass the 
// connection to them for each function
pub struct Database {

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

    // gets the id and a given column from the given table
    // and returns it in a vector of tuples (id, value)
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
        
    // gets the given object by id and returns it
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

    pub fn get_projectile(&self, id: i32) -> Projectile {
            
        let new_projectile = self.conn.query_row("SELECT * FROM projectile WHERE projectile_id = ?1", params![id], |row| {
            Ok(Projectile {
                projectile_id: row.get("projectile_id")?,
                casing_id: row.get("casing_id")?,
                manufacturer: row.get("manufacturer")?,
                diameter: row.get("diameter")?,
                weight: row.get("weight")?,
                projectile_type: row.get("type")?,
                length: row.get("length")?,
                sectional_density: row.get("sectional_density")?,
            })
        }).unwrap();

        new_projectile
    }

    pub fn get_powder(&self, id: i32) -> Powder {
            
        let new_powder = self.conn.query_row("SELECT * FROM powder WHERE powder_id = ?1", params![id], |row| {
            Ok(Powder {
                powder_id: row.get("powder_id")?,
                manufacturer: row.get("manufacturer")?,
                powder_type: row.get("type")?,
            })
        }).unwrap();

        new_powder
    }

    pub fn get_load(&self, id: i32) -> Load {
            
        let new_load = self.conn.query_row("SELECT * FROM loads WHERE load_id = ?1", params![id], |row| {
            Ok(Load {
                load_id: row.get("load_id")?,
                powder_id: row.get("powder_id")?,
                casing_id: row.get("casing_id")?,
                projectile_id: row.get("projectile_id")?,
                powder_weight: row.get("powder_weight")?,
                primer_make: row.get("primer_make")?,
                primer_lot: row.get("primer_lot")?,
                headstamp: row.get("headstamp")?,
                brass_lot: row.get("brass_lot")?,
                trim_to_length: row.get("trim_to_length")?,
                cartridge_overall_length: row.get("cartridge_overall_length")?,
                crimp_diameter: row.get("crimp_diameter")?,
            })
        }).unwrap();

        new_load
    }

    pub fn get_ballistic_test(&self, id: i32) -> BallisticTest {
            
        let new_test = self.conn.query_row("SELECT * FROM tests WHERE test_id = ?1", params![id], |row| {
            Ok(BallisticTest {
                test_id: row.get("test_id")?,
                load_id: row.get("load_id")?,
                air_pressure: row.get("air_pressure")?,
                altitude: row.get("altitude")?,
                air_temperature: row.get("air_temperature")?,
                wind_speed: row.get("wind_speed")?,
                wind_direction: row.get("wind_direction")?,
                barrel_length: row.get("barrel_length")?,
                twist_rate: row.get("twist_rate")?,
                distance_to_target: row.get("distance_to_target")?,
                date: row.get("date")?,
            })
        }).unwrap();

        new_test
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
                        (4, 0.356),
                        (5, 0.357)], pairs);
    }

    #[test]
    fn test_get_functions() {

        let database = Database::new("./loaddata.db");
        let test_casing = database.get_casing(1);
        let test_projectile = database.get_projectile(1);
        let test_powder = database.get_powder(2);
        let test_ballistic_test = database.get_ballistic_test(1);
        let test_load = database.get_load(1);

        assert!(test_casing.casing_id == 1); 
        assert!(test_casing.name == ".357 Magnum".to_string());
        assert!(test_casing.primer_size == "SPM".to_string());
        assert!(test_casing.case_type == "Rimmed, straight".to_string());
        assert!(test_casing.max_psi == 35000.0);
        assert!(test_casing.max_cup == 45000.0);

        assert!(test_projectile.projectile_id == 1);
        assert!(test_projectile.casing_id == 4);
        assert!(test_projectile.manufacturer == "Berry's".to_string());
        assert!(test_projectile.diameter == 0.356);
        assert!(test_projectile.weight == 124.0);
        assert!(test_projectile.projectile_type == "Round Nose Plated".to_string());
        assert!(test_projectile.length == 0.6);
        assert!(test_projectile.sectional_density == 0.139);

        assert!(test_powder.powder_id == 2);
        assert!(test_powder.manufacturer == "Hodgdon".to_string());
        assert!(test_powder.powder_type == "TiteGroup".to_string());


    }
}
