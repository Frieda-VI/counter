use std::{cmp::Ordering, fs};

mod args;
mod utils;

use args::*;

const FILE_NAME: &str = "history.txt";
const QUIT: &str = "q!";

fn init() {
    let data_file = fs::read(FILE_NAME);

    if let Err(_) = data_file {
        utils::reset_data();
    }

    println!("Running...");
}

fn main() {
    init();

    loop {
        let arguments = Arguments::get_args();
        if arguments.command.cmp(&QUIT.to_string()) == Ordering::Equal {
            break;
        }

        match arguments.command.as_str() {
            "add" => arguments.add(),
            "remove" => arguments.remove(),
            "reset" => arguments.reset(),
            _ => arguments.invalid(),
        }
    }
}
