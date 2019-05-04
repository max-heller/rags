use dirs;
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
    let mut commands: Vec<String> = vec![];
    for line in f.lines() {
        if let Some(cmd) = line?.split(';').nth(1) {
            commands.push(cmd.to_string());
        }
    }
    Ok(commands)
}
