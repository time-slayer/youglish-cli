use std::env;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        usage();
        process::exit(1);
    }

    let phrase = arguments[1..].join("_");
    let url = format!("https://youglish.com/pronounce/{}/english", phrase);
    open::that(url).unwrap();
}

fn usage() {
    println!(
        "\
Usage: yg <phrase>

Example: yg literally engage"
    );
}
