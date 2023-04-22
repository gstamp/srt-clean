use regex::Regex;
use srtlib::Subtitles;
use std::{env, process};

fn main() {
    if env::args().len() != 2 {
        println!("Usage: srt-clean {{srt-file}}");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    let mut subs = Subtitles::parse_from_file(input_path, None).unwrap();

    for s in &mut subs {
        let braces_regex = Regex::new(r"（.*?）").unwrap();
        let braces_regex2 = Regex::new(r"\(.*?\)").unwrap();

        let text = &s.text;
        let cleaned_text = braces_regex.replace_all(text, "").to_string();
        let cleaned_text = braces_regex2.replace_all(&cleaned_text, "").to_string();
        s.text = cleaned_text;
    }

    subs.write_to_file(input_path, None)
        .expect("Unable to overwrite input file")
}
