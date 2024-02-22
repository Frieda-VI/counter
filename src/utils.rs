use std::{fs, io, time::SystemTime};

const FILE_NAME: &str = "history.txt";

pub fn split_string(s: &String) -> Vec<String> {
    s.split_whitespace()
        .map(|word| String::from(word))
        .collect()
}

pub fn merge_str(u: usize, v: usize, array: &Vec<String>) -> String {
    let mut combined = String::new();

    for i in u..v {
        combined.push_str(array[i].as_str());
        combined.push_str(" ");
    }

    combined
}

pub fn get_input() -> String {
    let mut reader = String::new();
    io::stdin()
        .read_line(&mut reader)
        .expect("Failed to read input");

    reader
}

pub fn get_timestap() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to read time")
        .as_secs()
}

pub fn get_data() -> String {
    fs::read_to_string(FILE_NAME).expect("Failed to read history")
}

pub fn reset_data() {
    let _ = fs::write(FILE_NAME, "");
}
