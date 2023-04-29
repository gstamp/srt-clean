use glob::glob;
use regex::Regex;
use srtlib::Subtitles;
use std::{env, process};

fn main() {
    // iterate over a list of files provided as command line arguments
    // for each file, parse the file and print it to stdout
    // if no files are provided, print usage information

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: srt-clean <file1> [<file2> ...]");
        process::exit(1);
    }
    for filename_arg in &args[1..] {
        for path in glob(filename_arg).unwrap() {
            let filepath = path.unwrap();
            let filename = filepath.to_str().unwrap();
            println!("Cleaning {}", filename);
            let mut subs = Subtitles::parse_from_file(filepath.as_path(), None).unwrap();

            for s in &mut subs {
                let braces_regex = Regex::new(r"（.*?）").unwrap();
                let braces_regex2 = Regex::new(r"\(.*?\)").unwrap();

                let text = &s.text;
                let cleaned_text = braces_regex.replace_all(text, "").to_string();
                let cleaned_text = braces_regex2.replace_all(&cleaned_text, "").to_string();
                s.text = cleaned_text;
            }

            subs.write_to_file(filename, None)
                .expect("Unable to overwrite input file")
        }
    }
}
