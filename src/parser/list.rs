pub fn compile(line: &String) -> String {
    let (dot, content) = line.split_once(" ").unwrap();
    let list_type = if dot.chars().nth(0).unwrap().is_numeric() {
        vec!["<ol>\n", "</ol>"]
    } else {
        vec!["<ul>\n", "</ul>"]
    };

    let tags: Vec<&str> = vec!["<li>", "</li>"];

    let output_line: String = format!("{}{}{content}{}{}\n", list_type[0], tags[0], tags[1], list_type[1]);

    output_line
}