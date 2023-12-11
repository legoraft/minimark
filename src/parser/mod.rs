use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod heading;
mod paragraph;
mod list;

pub fn parse(filename: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec!["<html>\n".to_string(), "<body>\n\n".to_string()];
    let mut html_line = String::new();

    let input_filename = Path::new(filename);
    let file = File::open(input_filename).expect("Error, failed to read file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.unwrap();
        let first_char: char = match line_content.chars().nth(0) {
            Some(c) => c,
            None => continue,
        };
        let output_line: String;

        match first_char {
            '#' => output_line = heading::compile(&line_content),
            '-' => output_line = list::compile(&line_content),
            _ => output_line = {
                if first_char.is_numeric() {
                    list::compile(&line_content)
                } else {
                    paragraph::compile(&line_content)
                }
            },
        };

        html_line += &*output_line;
    }

    html_line = html_line.replace("</ul>\n<ul>", "").replace("</ol>\n<ol>", "");
    let lines: Vec<String> = html_line.split("\n").map(|s| s.to_string()).collect();

    for line in lines {
        tokens.push(line + "\n");
    }

    tokens.push("</body>\n".to_string());
    tokens.push("</html>".to_string());

    tokens
}