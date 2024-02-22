use std::{fs, process};

use crate::utils::*;

const FILE_NAME: &str = "history.txt";

pub struct Arguments {
    pub command: String,
    pub comments: String,
}

impl Arguments {
    fn new(args: Vec<String>) -> Arguments {
        Arguments {
            command: args[0].clone(),
            comments: || -> String { merge_str(1, args.len(), &args) }(),
        }
    }

    pub fn get_args() -> Arguments {
        let args = split_string(&get_input());

        if args.len() < 1 {
            println!("One or more arguments are required in order to proceed");
            process::exit(1);
        }

        Arguments::new(args)
    }
}

impl Arguments {
    pub fn add(&self) {
        let timestamp = get_timestap();
        let message = format!("\n{}: {}", timestamp, self.comments);

        let mut data = get_data();
        data.push_str(&message);

        let _ = fs::write(FILE_NAME, data);
    }

    pub fn remove(&self) {
        let data = get_data();
        let lines: Vec<&str> = data.as_str().lines().collect();

        let data = if lines.len() > 1 {
            lines[..lines.len() - 1].join("\n")
        } else {
            data.to_owned()
        };

        let _ = fs::write(FILE_NAME, data);
    }

    pub fn reset(&self) {
        if self.comments.as_str() == "confirmed " {
            reset_data();
        } else {
            println!("Could not confirm data reset");
        }
    }

    pub fn invalid(&self) {
        println!("Invalid command: {}", self.command);
    }
}
