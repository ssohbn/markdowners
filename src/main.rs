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
        println!("{:?}", &tag);
        tags.push(tag);
    }
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
        let mut chars = start.chars();

        let first_thingy = chars.next().unwrap();
        
        let text = text.to_owned();
        match first_thingy {
            '#' => {
                let mut the_juice = 1;
                while chars.next().is_some() && chars.next() == Some('#') {
                    the_juice += 1;
                }
                return Tag::Header { text, number: the_juice};
            },
            '>' => {
                return Tag::BlockComment { text };
            },
            '-' => {
                return Tag::UnorderedListItem { text }
            },
            thingy if ('0'..'9').any(|n| n == thingy) => {
                return Tag::OrderedListItem { text, index: start.parse().unwrap() }
            },
            '\t' => return Tag::Code{ text },

            _ => return Tag::Paragraph{ text },
        }
    }
    return Tag::Header { text: "hello".to_owned(), number: 6 }
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


