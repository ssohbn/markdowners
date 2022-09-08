//! stuff



use clap::Parser;
mod stuff;
use stuff::{
    Args,
    read_file,
    markdown_to_html,
};

fn main() {
    let args = Args::parse();
	let lines = read_file(&args.md_path);
    let output = markdown_to_html(&lines);
    std::fs::write(args.html_path, output).expect("couldnt write to file");
}



