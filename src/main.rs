mod sql;
use sql::Database;
use std::io;
use std::io::Write;

fn main() {
        
    let database = Database::new("./loaddata.db");

    let mut input = String::new();

    println!("Welcome to Reloader's Pal!");

    loop {

        println!("Menu");
        println!("1. See casing specifications");
        println!("2.");
        println!("3.");
        println!("4.");
        println!("5.");
        println!("6.");
        println!("type exit to quit");

        get_input(&mut input);

        match input.to_lowercase().trim() {

            "1" => {
                let pairs: Vec<(i32, String)> = database.get_id_pairs("casing", "name");
                let mut index = 0;
                for (id, name) in pairs.iter() {
                    println!("{}. {}", index, name);
                    index += 1;
                }
                println!("Please enter id");
                get_input(&mut input);
                let i = input.trim().parse::<i32>().unwrap();
                let (id, _) = pairs.get(i as usize).unwrap();
                let casing = database.get_casing(*id);
            }
            "exit" => break,
            _ => println!("Invalid option"), 

        };

    }

    println!("Goodbye!");
    
}

fn get_input(buffer: &mut String) {

    buffer.clear();

    print!(">> ");
    io::stdout()
        .flush()
        .expect("failed to flush buffer");

    io::stdin()
        .read_line(buffer)
        .expect("failed to read line");

}
