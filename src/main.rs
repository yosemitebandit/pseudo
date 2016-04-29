extern crate crypto;
extern crate getopts;
extern crate rustc_serialize;

use crypto::md5::Md5;
use crypto::digest::Digest;
use getopts::Options;
use rustc_serialize::json;

use std::env;
use std::fs::File;
use std::io::Read;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

#[derive(RustcEncodable)]
struct Request {
    contents: String,
    language: String,
    // Hash is the MD5 of contents + language.
    hash: String,
}

fn get_file_contents(input_path: &str) -> Result<String, String> {
    let mut file = try!(File::open(input_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    Ok(contents.trim().to_owned())
}

fn compose_request(contents: &str, language: &str, hash: &str) -> Result<String, String> {
    let request = Request {
        contents: contents.to_string(),
        language: language.to_string(),
        hash: hash.to_string(),
    };
    let encoded = try!(json::encode(&request).map_err(|e| e.to_string()));
    Ok(encoded)
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
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    let language = matches.opt_str("l");
    let input = if !matches.free.is_empty() {
        // grab any 'free string fragments' if there are any -- this is the input FILE
        matches.free[0].clone()
    } else {
        // if there aren't any string fragments, the program wasn't run correctly
        print_usage(&program, opts);
        return;
    };

    // read file contents
    let contents = match get_file_contents(&input) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // TODO: this is fragile
    let language = language.unwrap();

    // Get the MD5 of contents + language.
    let mut hasher = Md5::new();
    let hash_input = format!("{}{}", &contents, &language);
    hasher.input_str(&hash_input);
    let hash = hasher.result_str();

    // Compose message and jsonify it.
    let request = match compose_request(&contents, &language, &hash) {
        Ok(request) => request,
        Err(err) => {
            println!("Error: {}", err);
            return
        }
    };

    println!("{}", request);

    // send the message
    // poll for result..
    // save output
    // display result
}
