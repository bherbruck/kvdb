use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::env::args;
use std::fs::{read_to_string, write};
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let path = args().nth(1).expect("Database file not specified");

    // TODO: add an execute command that skips the shell

    let mut db = Kvdb::new(path);

    let mut rl = Editor::<()>::new().unwrap();

    loop {
        let readline = rl.readline("kvdb> ");
        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
        let KvdbCommand { verb, key, value } = parse_input(&input)?;
        match verb.to_lowercase().as_str() {
            "get" => {
                let value = db.get(&key).unwrap_or_default();
                println!("{value}");
            }
            "set" => match (&key, &value) {
                i if i.0.is_empty() || i.1.is_empty() => eprintln!("Error: input error"),
                _ => {
                    db.set(&key, &value);
                    db.flush();
                }
            },
            "del" => {
                db.del(&key);
                db.flush();
            }
            _ => (),
        }
    }
    Ok(())
}

struct KvdbCommand {
    verb: String,
    key: String,
    value: String,
}

fn parse_input(input: &str) -> Result<KvdbCommand, Error> {
    let args = input.split(' ').collect::<Vec<&str>>();
    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid input"));
    }
    let verb = args[0].trim().to_string();
    let key = args[1].trim().to_string();
    let value = args[2..].join(" ").trim().to_string();
    Ok(KvdbCommand { verb, key, value })
}

fn parse_db_data(db_raw: &str) -> HashMap<String, String> {
    let db = db_raw.lines().fold(HashMap::new(), |mut acc, line| {
        let (key, value) = line.split_once(' ').unwrap();
        acc.insert(key.to_string(), value.to_string());
        acc
    });
    db
}

struct Kvdb {
    path: String,
    map: HashMap<String, String>,
}

impl Kvdb {
    fn new(path: String) -> Kvdb {
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

    fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).map(|value| value.to_string())
    }

    fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    fn del(&mut self, key: &str) {
        self.map.remove(key);
    }

    fn flush(&mut self) {
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
