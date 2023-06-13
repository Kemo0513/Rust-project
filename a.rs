use std::{io::stdin, process::exit};
fn main() {
    let mut buf = String::new();

    if let Err(err) = stdin().read_line(&mut buf) {
        eprintln!("{}", err.to_string());
        exit(1);
    }

    println!("{buf}");
}
