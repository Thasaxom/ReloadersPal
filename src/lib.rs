use rusqlite::{Connection, Result, OpenFlags, params};
use std::process;

struct Load {
    load_id: i32,
    powder_id: i32,
    casing_id: i32,
    projectile_id: i32,
    powder_weight: f64,
    primer_make: String,
    primer_lot: String,
    headstamp: String,
    brass_lot: String,
    trim_to_length: f64,
    cartridge_overall_length: f64,
    crimp_diameter: f64,
}

struct BallisticTest {
    test_id: i32,
    load_id: i32,
    air_pressure: f64,
    altitude: f64,
    air_temperature: f64,
    wind_speed: f64,
    wind_direction: String,
    barrel_length: f64,
    twist_rate: f64,
    distance_to_target: f64,
    date: String,
}

struct Casing {
    casing_id: i32,
    name: String,
    primer_size: String,
    case_type: String,
    max_psi: f64,
    max_cup: f64,
}

impl Casing {
    
    fn new(name: &str) -> Casing {
    
        let conn = open_connection();

        let new_casing = conn.query_row("SELECT * FROM casing WHERE name = ?1", params![name], |row| {
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

struct Projectile {
    projectile_id: i32,
    casing_id: i32,
    manufacturer: String,
    diameter: f64,
    weight: f64,
    projectil_type: String,
    length: f64,
    sectional_density: f64,
}

struct Powder {
    powder_id: i32,
    manufacturer: String,
    powder_type: String,
}

fn open_connection() -> Connection {
    
        let conn = Connection::open_with_flags("./loaddata.db", OpenFlags::SQLITE_OPEN_READ_WRITE);

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_casing() {

        let test_casing = Casing::new(".357 Magnum");

        assert_eq!(test_casing.casing_id, 1);
        assert_eq!(test_casing.name, ".357 Magnum");
        assert_eq!(test_casing.max_psi, 35000.0);

    }

}
