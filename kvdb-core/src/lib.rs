use std::collections::HashMap;
use std::fs::{read_to_string, write};

fn parse_db_data(db_raw: &str) -> HashMap<String, String> {
    let db = db_raw.lines().fold(HashMap::new(), |mut acc, line| {
        let (key, value) = line.split_once(' ').unwrap();
        acc.insert(key.to_string(), value.to_string());
        acc
    });
    db
}

pub struct Kvdb {
    path: String,
    map: HashMap<String, String>,
}

impl Kvdb {
    pub fn new(path: String) -> Kvdb {
        let contents = match read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => {
                let contents = String::new();
                write(&path, &contents).unwrap();
                contents
            }
        };

        let map = parse_db_data(&contents);

        Kvdb { path, map }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).map(|value| value.to_string())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    pub fn del(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn flush(&mut self) {
        let db_raw = self
            .map
            .iter()
            .fold(String::new(), |mut acc, (key, value)| {
                let line = format!("{} {}\n", key, value);
                acc.push_str(&line);
                acc
            });
        write(&self.path, db_raw).expect("Could not write to db");
    }
}
