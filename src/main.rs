//! stuff

use clap::Parser;
mod lib; 
use lib::{
    Args,
    markdown_to_html,
};

fn main() {
    let args = Args::parse();
    let output = markdown_to_html(&args.md_path);
    std::fs::write(args.html_path, output).expect("couldnt write to file");
}



