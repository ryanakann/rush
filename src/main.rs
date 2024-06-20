use std::io::{stdin, stdout, Error, Write};

fn main() {
    let ps1 = "> ";
    let mut input = String::new();

    loop {
        print!("{}", ps1.to_string());
        flush();
        read_input(&mut input);
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

fn read_input(input: &mut String) {
    match stdin().read_line(input) {
        Ok(_) => {},
        Err(e) => {
            failure(e);
        },
    }
}

fn failure(e: Error) {
    eprintln!("Command failed: {}", e);
}
