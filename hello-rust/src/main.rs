use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    // Stdout for printing messages in console
    let stdout = stdout();
    let message = String::from("Hello world and welcome to Rust lang!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
