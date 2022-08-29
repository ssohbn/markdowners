static MD_PATH: &'static str = "md_file.md";

use std::fs;
use regex::Regex;

fn main() {
    let lines = read_file(MD_PATH);
    let lines = lines.lines().map(|line| line.to_owned());

    lines.for_each( |line| {
        parse_markdown(&line);
    });
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("couldnt read file");
    contents
}

fn parse_markdown(line: &str) -> String {

    if !line.contains(' ') {
        return "<br>".to_string();
    }

    let (start, text) = line.split_at(line.find(' ').unwrap());
    let re = Regex::new("[A-Za-z]").unwrap(); 

    // safe to say that non a-z is probably
    // a tag of some sort
    if !re.is_match(start) {
        let first_thingy = start.get(0..1);
        match first_thingy {
            Some("#") => println!("header"),
            Some(">") => println!("block quote"),
            Some("-") => println!("unordered list"),
            tingy if Regex::new("[0-9].").unwrap().is_match(tingy.unwrap()) => {
                
            }
            Some("\t") =>  println!("code"),
//           Some('0'..='9') => println!("ordered list"),

            None => panic!("im not sure this is reachable"),
            Some(_) => panic!("i think thhis is unreachable"),
        }
    }

   return format!("<p>{}</p>", text);
}

