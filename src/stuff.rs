use regex::Regex;
use clap::Parser;


/// call this if u wanna do the um progrma library style wahoo this is like the whole thing
pub fn markdown_to_html(md_path: &str) -> String {
    let file_content = read_file(md_path);
    let tags = parse_markdown(file_content);
    let output = parse_tags(tags);
    output
}

pub fn read_file(path: &str) -> String {
	let contents = std::fs::read_to_string(path).expect("couldnt read file");
	contents
}

fn parse_tags(tags: Vec<Tag>) -> String {
	let mut output: Vec<String> = Vec::new();
	let mut p_tags = tags.iter().peekable();

	let mut in_block = false;
	while p_tags.peek().is_some() {
		match p_tags.next().unwrap() {
			Tag::BlockComment { text } => {
				output.push(format!("<blockquote>{}</blockquote>", text).to_string())
			},
			Tag::Break {} => output.push("<br>".to_string()),
			Tag::Header { text, number } => {
				output.push(format!("<h{}>{}</h{}>", number, text, number).to_string())
			},
			Tag::Paragraph { text } => output.push(format!("<p>{}</p>", text)),
			Tag::OrderedListItem { text } => {
				let mut ol: String = String::new();

				if !in_block {
					ol.push_str("<ol>");
					in_block = true;
				}

				ol.push_str(&format!("<li>{}</li>", text));

				if !matches!(p_tags.peek(), Some(&Tag::OrderedListItem { text: _ })) {
					ol.push_str("</ol>");
					in_block = false;
				}

				output.push(ol);
			},
			Tag::Code { text } => {
				let mut code: String = String::new();

				if !in_block {
					code.push_str("<pre><code>");
				}
				code.push_str(&text);

				if !matches!(p_tags.peek(), Some(&Tag::Code { text: _ })) {
					code.push_str("</pre></code>");
				}

				output.push(code);
			},
			Tag::UnorderedListItem { text } => {
				let ul = unordered_list_html(&mut in_block, &text.trim(), &mut p_tags);

				output.push(ul);
			},
		}
	}
	output.join("\n")
}

fn unordered_list_html(
	in_block: &mut bool,
	text: &str,
	p_tags: &mut std::iter::Peekable<std::slice::Iter<Tag>>,
) -> String {
	let mut ul: String = String::new();

	if !*in_block {
		ul.push_str("<ul>");
		*in_block = true;
	}

	ul.push_str(&format!("<li>{}</li>", text));

	if !matches!(p_tags.peek(), Some(&Tag::UnorderedListItem { text: _ })) {
		ul.push_str("</ul>");
		*in_block = false;
	}

	ul
}

fn parse_markdown(lines: String) -> Vec<Tag> {
	let mut tags: Vec<Tag> = Vec::new();
	let lines = lines.lines().map(|line| line.to_owned());

	for line in lines {
		if !line.contains(' ') {
			// should actually just check if it is empty.
			// this is going to error soon enough
			tags.push(Tag::Break {});
		} else {
			let (start, text) = line.split_at(line.find(' ').expect("NO SPLIT BIG FAIL"));
			let re = Regex::new("[A-Za-z]").unwrap();

			// safe to say that non a-z is probably
			// a tag of some sort
			if !re.is_match(start) {
				let mut chars = start.chars().peekable();

				let first_thingy = chars.peek().unwrap();

				let text = text.to_owned();
				match first_thingy {
					'#' => {
						tags.push(header(&mut chars, text));
					},
					'>' => {
						tags.push(Tag::BlockComment { text });
					},
					'-' => tags.push(Tag::UnorderedListItem { text }),
					thingy if ('0'..'9').any(|n| &n == thingy) => {
						tags.push(Tag::OrderedListItem { text });
                    }

					// this is an error waiting to happen. i do not want to fix it.
					// post test: the error happened. i have to fix it.
					'\\' => tags.push(Tag::Code { text }),

					_ => panic!("not gonna happen"),
				}
			} else {
				tags.push(Tag::Paragraph { text: line });
			}
		}
	}

	tags
}


fn header(chars: &mut std::iter::Peekable<std::str::Chars>, text: String) -> Tag {
	let mut the_juice = 1;
	while chars.next() == Some('#') && chars.peek().is_some() {
		the_juice += 1;
	}
	return Tag::Header {
		text,
		number: the_juice,
	};
}

#[derive(Debug)]
enum Tag {
	BlockComment { text: String },
	Header { text: String, number: u8 },
	UnorderedListItem { text: String },
	OrderedListItem { text: String },
	Code { text: String },
	Paragraph { text: String },
	Break {},
}

#[cfg(test)]
pub mod tests {
    use super::*;

	#[test]
	fn header_test() {
		let markdown_line = "# line".to_string();
		let (start, text) = markdown_line.split_at(markdown_line.find(' ').unwrap());
		let tag = header(&mut start.chars().peekable(), text.to_string());

		if let Tag::Header { text: _, number } = tag {
			assert_eq!(number, 1)
		}

		let markdown_line = "## line".to_string();
		let (start, text) = markdown_line.split_at(markdown_line.find(' ').unwrap());
		let tag = header(&mut start.chars().peekable(), text.to_string());

		if let Tag::Header { text: _, number } = tag {
			assert_eq!(number, 2)
		}

		let markdown_line = "### line".to_string();
		let (start, text) = markdown_line.split_at(markdown_line.find(' ').unwrap());
		let tag = header(&mut start.chars().peekable(), text.to_string());

		if let Tag::Header { text: _, number } = tag {
			assert_eq!(number, 3)
		}

		let markdown_line = "#### line".to_string();
		let (start, text) = markdown_line.split_at(markdown_line.find(' ').unwrap());
		let tag = header(&mut start.chars().peekable(), text.to_string());

		if let Tag::Header { text: _, number } = tag {
			assert_eq!(number, 4)
		}

		let markdown_line = "##### line".to_string();
		let (start, text) = markdown_line.split_at(markdown_line.find(' ').unwrap());
		let tag = header(&mut start.chars().peekable(), text.to_string());

		if let Tag::Header { text: _, number } = tag {
			assert_eq!(number, 5)
		}

		let markdown_line = "###### line".to_string();
		let (start, text) = markdown_line.split_at(markdown_line.find(' ').unwrap());
		let tag = header(&mut start.chars().peekable(), text.to_string());

		if let Tag::Header { text: _, number } = tag {
			assert_eq!(number, 6)
		}
	}
}

#[derive(Parser)]
pub struct Args {
    #[clap(long, short)]
    pub html_path: String,

    #[clap(long, short)]
    pub md_path: String,
}
