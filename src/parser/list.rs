pub fn compile(line: &String) -> String {
    let (_, content) = line.split_once(" ").unwrap();

    let tags: Vec<&str> = vec!["<li>", "</li>"];

    let output_line: String = format!("{}{content}{}\n", tags[0], tags[1]);

    output_line
}