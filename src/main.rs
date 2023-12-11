mod parser;

use std::env::args;
use std::fs::File;
use std::io::Write;

fn main() {
    let arguments: Vec<String> = args().collect();

    match &arguments.len() {
        2 => compile_file(&arguments[1]),
        _ => usage(),
    }
}

fn compile_file(filename: &str) {
    let output_filename = String::from(format!("{}.html", &filename[..&filename.len() - 3]));
    let tokens = parser::parse(filename);

    let mut output_file = File::create(&output_filename)
        .expect("Error, could not create output file!");

    for line in tokens.iter() {
        dbg!(&line);
        output_file.write_all(line.as_bytes()).expect("Error, failed to write line!");
    }
}

fn usage() {
    full_banner();
}

fn full_banner() {
    short_banner();

    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Usage: minimark <somefile>.md");
}

fn short_banner() {
    println!("{}", get_title());
}

fn get_title() -> String{
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title
}