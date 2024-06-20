use std::{
    env::set_current_dir, 
    io::{stdin, stdout, Error, Write},
    fs::read_dir
};

fn main() {
    let ps1 = "> ";

    loop {
        print!("{}", ps1.to_string());
        flush();
        let input = read_input();
        let mut tokens = input.split_whitespace();

        if let Some(command) = tokens.next() {
            let args: Vec<&str> = tokens.collect();
            parse(command, args);
        }
    }
}

fn parse(command: &str, args: Vec<&str>) {
    match command {
        "cd" => {
            if args.len() != 1 {
                eprintln!("cd: wrong number of arguments");
            } else {
                let path = args[0];
                if let Err(e) = set_current_dir(path) {
                    eprintln!("cd: {}", e);
                }
            }
        }
        "ls" => {
            
            let path = match args.len() {
                0 => ".".to_string(),
                _ => args[0].to_string(),
            };
            let files = read_dir(path).unwrap();
            for f in files {
                print!("{} ", f.unwrap().path().display());
            }
            println!("")
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn flush() {
    match stdout().flush() {
        Ok(_) => {},
        Err(e) => {
            failure(e);
        },
    }
}

fn read_input() -> String {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => {
            failure(e);
        },
    }
    return input.trim().to_string();
}

fn failure(e: Error) {
    eprintln!("Command failed: {}", e);
}
