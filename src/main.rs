use clap::Parser;

mod stuff;
use stuff::{
    Args,
    read_file,
    do_magic,
};

fn main() {
	// let mut file = std::fs::OpenOptions::new().append(true).open(HTML_PATH);
    let args = Args::parse();
	let lines = read_file(&args.md_path);
    let output = do_magic(&lines);
    
    std::fs::write(args.html_path, output).expect("couldnt write to file");
}



