extern crate crypto;
extern crate getopts;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use std::env;
use std::fs::File;
use std::io::Read;

use crypto::md5::Md5;
use crypto::digest::Digest;
use getopts::Options;
use hyper::Client;
use rustc_serialize::json;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


#[derive(RustcEncodable)]
struct Request {
    contents: String,
    language: String,
    // The hash is the MD5 of contents + language.
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


fn post_json(url: &str, payload: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.post(url).body(payload).send());
    let mut buffer = String::new();
    try!(response.read_to_string(&mut buffer));
    Ok(buffer)
}


fn http_get(url: &str) -> Result<String, String> {
    let client = Client::new();
    let mut response = client.get(url)
        .send()
        .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    Ok(body)
}


fn main() {
    // parse args
    let args: Vec<String> = env::args().collect();
    // get the path of the binary
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "OUTPUT");
    opts.optopt("l", "language", "set output language", "LANGUAGE");
    opts.optflag("h", "help", "print this");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = match matches.opt_str("o") {
        Some(v) => v,
        None => {
            println!("Error: please specify an output file.");
            return;
        }
    };
    let language = match matches.opt_str("l") {
        Some(v) => v,
        None => {
            println!("Error: please specify a language.");
            return;
        }
    };
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
            return;
        }
    };

    let code_submission_url = "http://requestb.in/oblkqhob";
    match post_json(&code_submission_url, &request) {
        Ok(_) => println!("request sent to the cloud compiler.."),
        Err(err) => println!("Error: {}", err),
    };

    // poll for result..
    let result_polling_base_url = "http://oakmachine.com";
    let result_polling_url = format!("{}/{}", result_polling_base_url, "neovim");
    let response = match http_get(&result_polling_url) {
        Ok(response) => {
            response
        },
        Err(err) => {
            println!("Error: {}", err);
            return
        }
    };

    println!("{}", response);


    // save output
    // display result
}
