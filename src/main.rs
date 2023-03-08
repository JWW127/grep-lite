use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    grep_lite()
}

// 'reader' which can be 'input<file>' || 'stdin' will be processed
// into memory as a Buffer then grepped then printed
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("\n{}: {}\n", i + 1, line),
            None => (),
        }
    }
}

fn grep_lite() {
    // provides cli arg capabilities in terminal
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern") // value "pattern" to accept
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input") // value of "input" to accept
                .help("File to Search")
                .takes_value(true)
                .required(true),
        )
        .get_matches(); //parses input/stdin

    /*
    unwraps the value of 'pattern' if argument provided
    if not provided app crashes provides reason and proper USAGE
    */
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    // same as 'pattern' but doesnt crash instead uses '-' as value
    let input = args.value_of("input").unwrap_or("-");

    // if input wasnt valid run this
    if input == "-" {
        // get stdin
        let stdin = io::stdin();
        // lock stdin for reading
        let reader = stdin.lock();
        // call process_lines on it
        process_lines(reader, re);
    } else {
        // called if input file provided
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
