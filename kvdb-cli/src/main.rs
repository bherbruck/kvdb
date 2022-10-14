use kvdb_core::Kvdb;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::args;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let path = args().nth(1).expect("Database file not specified");

    let mut db = Kvdb::new(path);
    
    let command_string = args().nth(2);

    // if a command was specified, execute it and exit
    if let Some(command_string) = command_string {
        let command = parse_input(&command_string)?;
        execute_command(&mut db, command);
        return Ok(());
    }

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

        let command = parse_input(&input).unwrap_or_default();
        execute_command(&mut db, command);
    }
    Ok(())
}

fn execute_command(db: &mut Kvdb, command: KvdbCommand) {
    let KvdbCommand { verb, key, value } = command;
    match verb.to_lowercase().as_str() {
        "get" => {
            let value = db.get(&key).unwrap_or_default();
            println!("{value}");
        }
        "set" => match (&key, &value) {
            i if i.0.is_empty() || i.1.is_empty() => (),
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

struct KvdbCommand {
    verb: String,
    key: String,
    value: String,
}

impl Default for KvdbCommand {
    fn default() -> Self {
        KvdbCommand {
            verb: "".to_string(),
            key: "".to_string(),
            value: "".to_string(),
        }
    }
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
