use dirs;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_history(path: Option<std::path::PathBuf>) -> io::Result<Vec<String>> {
    let f = match path {
        Some(path) => File::open(path),
        None => {
            let mut path = dirs::home_dir().unwrap();
            path.push(".histfile");
            File::open(path)
        }
    }?;

    let f = BufReader::new(f);
    let re = Regex::new(r"^: \d+:\d;").unwrap();
    let commands: Vec<String> = f
        .lines()
        .filter_map(Result::ok)
        .map(|l| re.replace(&l, "").to_string())
        .collect();
    Ok(commands)
}
