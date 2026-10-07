use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let phrase = arguments[1..].join("_");

    let url = format!("https://youglish.com/pronounce/{}/english", phrase);
    open::that(url).unwrap();
}
