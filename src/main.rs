mod sql;

fn main() {

    let conn = sql::open_connection();

    let pairs: Vec<(i32, String)> = sql::get_id_pairs(&conn, "casing", "name");

    for (id, name) in pairs.iter() {
        println!("{} {}", id, name);
    }

}
