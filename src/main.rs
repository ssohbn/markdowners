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

fn parse_markdown(line: &str) -> &str {
    let re = Regex::new("[A-Za-z]").unwrap();
    
   let (start, text) = line.split_at(line.find(' ').unwrap());

   if !re.is_match(start) {
       // todo!("p tag")
       println!("{}", start);
   }


   return "temp"
}
