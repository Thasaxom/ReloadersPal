#![allow(dead_code)]
#![allow(unused_imports)]

mod sql;
mod reloading;

use std::env;
use std::process;
use sql::Database;

fn main() {
    
    let args = env::args();

    for arg in args {

        if arg.starts_with('-') {
            match arg.as_str() {

                "-h" | "--help" => print_help(),
                _ => println!("invalid option {}", arg),

            }
        }
        else {
            match arg.as_str() {

                _ => (),

            }
        }

    }

}

fn print_help() {

    println!("usage: reloader [options] [command] [args]");
    println!("options");
    println!("-h | --help: see this menu");
    process::exit(0);

}

