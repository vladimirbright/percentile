extern crate clap;

use clap::{Arg, App};
use std::cmp::Ordering::Equal;
use std::error::Error;
use std::f32;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;


fn print_results(mut timings: Vec<f32>) {
    timings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    let start: usize = 0;
    let end: usize = timings.len() - 1;
    if end == 0 {
        return;
    };
    let median: usize = end / 2;
    let p70: usize = end.checked_mul(70).unwrap().checked_div(100).unwrap();
    let p80: usize = end.checked_mul(80).unwrap().checked_div(100).unwrap();
    let p95: usize = end.checked_mul(95).unwrap().checked_div(100).unwrap();
    let p99: usize = end.checked_mul(99).unwrap().checked_div(100).unwrap();

    println!("");
    println!("Results:");
    println!("    Total count:   {}", timings.len());
    println!("    Min:           {}", timings[start]);
    println!("    Median:        {} ({})", timings[median], median + 1);
    println!("    70 percentile: {} ({})", timings[p70], p70 + 1);
    println!("    80 percentile: {} ({})", timings[p80], p80 + 1);
    println!("    95 percentile: {} ({})", timings[p95], p95 + 1);
    println!("    99 percentile: {} ({})", timings[p99], p99 + 1);
    println!("    Max:           {} ({})", timings[end], end + 1);
}

// This is the main function
fn main() {

    let matches = App::new("Calculate response time percentiles on logs")
        .version("0.0.1")
        .author("Vladimir Prokhoda <vladimirbright@gmail.com>")
        .arg(Arg::with_name("separator")
            .short("s")
            .long("separator")
            .default_value(" ")
            .help("Column separator")
            .takes_value(true))
        .arg(Arg::with_name("column")
            .short("c")
            .long("column")
            .default_value("8")
            .use_delimiter(false)
            .help("Column number to use")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
            .arg(Arg::with_name("print")
            .short("p")
            .long("print")
            .help("Print matched rows"))
        .get_matches();

    let sep = match matches.value_of("separator") {
        Some(f) => f,
        None => panic!("provide --separator option"),
    };
    let separator = match sep {
        "\\t" => "\t",
        _ => sep,
    };
    let column = match matches.value_of("column") {
        Some(f) => f,
        None => panic!("provide --column option"),
    };
    let timing_index: usize = match usize::from_str(column) {
        Ok(f) => f,
        Err(_) => panic!("--column must be integer"),
    };

	let file_path = matches.value_of("input").unwrap();
    let path = Path::new(&file_path);
    let display = path.display();

    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(f) => f,
    };
	let file = BufReader::with_capacity(128 * 1024, &f);

    let mut timings: Vec<f32> = vec![];

	for line in file.lines().filter_map(|result| result.ok()) {
		let v: Vec<&str> = line.split(separator).collect();
        let t = v[timing_index];

        if t == "-" {
            continue;
        }
		if matches.is_present("print") {
			println!("{}", line);
		}
        match f32::from_str(&t) {
            Err(_) => continue,
            Ok(f) => timings.push(f),
        };
    }

    print_results(timings);
}
