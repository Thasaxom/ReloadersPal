#![allow(dead_code)]
#![allow(unused_imports)]

mod sql;
mod reloading;

use std::env;
use std::process;
use sql::Database;

fn main() {
        
    for args in env::args() {

        match args.as_str() {

            "-h" | "--help" => print_help(),
            _ => "invalid option",

        }

    }

}

fn print_help() {

    println!("usage: reloader [options] [command] [args]");
    println!("options");
    println!("-h | --help: see this menu");

}

