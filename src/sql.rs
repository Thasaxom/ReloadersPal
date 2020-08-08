use rusqlite::{Connection, Result, OpenFlags, params};
use rusqlite::types::FromSql;
use std::process;

pub fn open_connection() -> Connection {

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

pub fn get_id_pairs<T: FromSql>(conn: &Connection, table: &str, column: &str) -> Vec<(i32, T)> {
    
    let mut query = String::from("SELECT * FROM ");
    query.push_str(table);
    let mut stmt = conn.prepare(&query).unwrap();
    let rows = stmt.query_map(params![], |row| {
        Ok((row.get(0)?, row.get(column)?))
    }).unwrap();

    let mut pairs = Vec::new();
    for pair in rows {
       pairs.push(pair.unwrap());
    }

    pairs

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_id_pairs() {

        let conn = open_connection();
        let pairs = get_id_pairs(&conn, "casing", "name");

        assert_eq!(vec![(1, ".357 Magnum".to_string()),
                        (2, "5.56x45mm NATO".to_string()),
                        (3, ".45 ACP".to_string()),
                        (4, "9x19mm Parabellum".to_string()),
                        (5, ".45-70 Government".to_string())], pairs);

        let pairs = get_id_pairs(&conn, "projectile", "diameter");

        assert_eq!(vec![(1, 0.356),
                        (2, 0.355),
                        (3, 0.458),
                        (5, 0.357)], pairs);
    }
}
