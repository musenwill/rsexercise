use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(cfg) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
