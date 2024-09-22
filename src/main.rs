mod marshall;

use marshall::marshall;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();


    let mut log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/home/moamen/iqraalabs/tests/emmet-language-server.log")
        .unwrap();


    loop {
        match marshall(&mut stdin.lock(), &mut log_file) {
            Some(body) => println!("message: {}", body),
            None => eprintln!("could not parse message"),
        }
    }
}
