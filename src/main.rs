extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Read;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn get_file_contents(input_path: &str) -> Result<String, String> {
    let mut file = try!(File::open(input_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    Ok(contents.trim().to_owned())
}

fn main() {
    // parse args
    let args: Vec<String> = env::args().collect();
    // get the path of the binary
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("l", "lang", "set output language", "LANG");
    opts.optflag("h", "help", "print this");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    let language = matches.opt_str("l");
    let input = if !matches.free.is_empty() {
        // grab any 'free string fragments' as the input if there are any
        matches.free[0].clone()
    } else {
        // if there aren't any string fragments, the program wasn't run correctly
        print_usage(&program, opts);
        return;
    };

    // read file contents
    match get_file_contents(&input) {
        Ok(contents) => println!("file contents: {}", contents),
        Err(err) => println!("Error: {}", err),
    }
    // compose message
    // hash it
    // send the message
    // poll for result..
    // save output
    // display result
}
