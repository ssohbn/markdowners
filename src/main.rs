static MD_PATH: &'static str = "md_file.md";
static HTML_PATH: &'static str = "html_file.html";

use std::fs;
use regex::Regex;

fn main() {
    let lines = read_file(MD_PATH);
    let lines = lines.lines().map(|line| line.to_owned());

    // let mut file = std::fs::OpenOptions::new().append(true).open(HTML_PATH);

    let mut tags: Vec<Tag> = Vec::new();
    for line in lines {
        let tag = parse_markdown(line);
//        println!("{:?}", &tag);
        tags.push(tag);
    }
    let output = parse_tags(tags);
    println!("the output: {}", output);
}

fn parse_tags(tags: Vec<Tag>) -> String {
    let mut output: Vec<String> = Vec::new();
    let mut p_tags = tags.iter().peekable();
    while p_tags.peek().is_some() {
        let in_block = false;
        match p_tags.next().unwrap() {
            Tag::BlockComment { text } => output.push(format!("<blockquote>{}</blockquote>", text).to_string()),
            Tag::Break {  } => output.push("<br>".to_string()),
            Tag::Header { text, number } => output.push(format!("<h{}>{}</h{}>", number, text, number).to_string()),
            Tag::Code { text } => {
                let mut code: String = String::new();

                if !in_block {
                    code.push_str("<pre><code>");
                }
                code.push_str(text);

                if !matches!(p_tags.peek(), Some(&Tag::Code{text: _})) {
                    code.push_str("</pre></code>");
                }

                output.push(code);
                },
                Tag::Paragraph { text } => output.push(format!("<p>{}</p>", text)),
                Tag::OrderedListItem { text, index } => {
                    todo!("this dont work");
                let mut ol: String = String::new();

                if !in_block {
                    ol.push_str("<ol>");
                }
                ol.push_str(&format!("<li>{}<li>", text));

                if !matches!(p_tags.peek(), Some(&Tag::Code{text: _})) {
                    ol.push_str("</ol>");
                }

                output.push(ol);

                },
                Tag::UnorderedListItem{ text } => {
                    todo!("this dont work");
                let mut ul: String = String::new();

                if !in_block {
                    ul.push_str("<ul>");
                }
                ul.push_str(text);

                if !matches!(p_tags.peek(), Some(&Tag::Code{text: _})) {
                    ul.push_str("</ul>");
                }

                output.push(ul);
                },
            }
        }

    output.join("\n")
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("couldnt read file");
    contents
}

fn parse_markdown(line: String) -> Tag {

    if !line.contains(' ') {
        return Tag::Break {  };
    }

    let (start, text) = line.split_at(line.find(' ').unwrap());
    let re = Regex::new("[A-Za-z]").unwrap(); 

    // safe to say that non a-z is probably
    // a tag of some sort
    if !re.is_match(start) {
        let mut chars = start.chars().peekable();

        let first_thingy = chars.next().unwrap();
        
        let text = text.to_owned();
        match first_thingy {
            '#' => {
                return header(&mut chars, text);
            },
            '>' => {
                return Tag::BlockComment { text };
            },
            '-' => {
                return Tag::UnorderedListItem { text }
            },
            thingy if ('0'..'9').any(|n| n == thingy) => {
                let (number, _) = start.split_at(start.find(".").unwrap());
                return Tag::OrderedListItem { text, index: number.parse().unwrap() }
            },

            // this is an error waiting to happen. i do not want to fix it.
            // post test: the error happened. i have to fix it.
            '\\' => return Tag::Code{ text },

            _ => panic!("not gonna happen"),
        }

    }
    return Tag::Paragraph { text: line };
}

fn header(chars: &mut std::iter::Peekable<std::str::Chars>, text: String) -> Tag {
    let mut the_juice = 1;
    while chars.peek().is_some() && chars.next() == Some('#') {
        the_juice += 1;
    }
    return Tag::Header { text, number: the_juice};
}

#[derive(Debug)]
pub enum Tag {
    BlockComment{text: String},
    Header{text: String, number: u8},
    UnorderedListItem{text: String},
    OrderedListItem{text: String, index: u8},
    Code{text: String},
    Paragraph{text: String},
    Break{},
}

#[cfg(test)]
pub mod tests {
    use crate::*;
    fn header_test() {
        todo!("make the tests!!!!!");
        header();
    }

}



