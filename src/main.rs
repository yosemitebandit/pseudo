extern crate getopts;

use getopts::Options;
use std::env;


fn do_work(input: &str, output: Option<String>, language: Option<String>) {
    println!("doing work!");
    println!("{}", input);
    let mut output_str = String::new();
    output_str = output.expect("fail!");
    let mut language_str = String::new();
    language_str = language.expect("fail!");
    println!("{} -> {}", output_str, language_str);
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {
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
    do_work(&input, output, language);
}
