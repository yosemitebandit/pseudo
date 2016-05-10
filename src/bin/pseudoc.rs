extern crate crypto;
extern crate dotenv;
extern crate getopts;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

extern crate pseudo;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crypto::md5::Md5;
use crypto::digest::Digest;
use dotenv::dotenv;
use getopts::Options;
use hyper::Client;
use rustc_serialize::json;

use pseudo::{CompilerRequest, CompilerResponse, print_usage};


const DEFAULT_ENDPOINT: &'static str = "http://pseudo-lang.oakmachine.com";
const COMPILE_ROUTE: &'static str = "compile";
const SECONDS_TO_SLEEP: i8 = 10;


fn get_file_contents(input_path: &str) -> Result<String, String> {
    let mut file = try!(File::open(input_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    Ok(contents.trim().to_owned())
}


fn compose_request(contents: &str, language: &str, hash: &str) -> Result<String, String> {
    let request = CompilerRequest {
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


fn http_get(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    Ok(body)
}


fn decode_response(response: &str) -> Result<CompilerResponse, String> {
    let response: CompilerResponse = try!(json::decode(&response).map_err(|e| e.to_string()));
    Ok(response)
}


fn write_file_contents(output_path: &str, contents: &str) -> Result<String, String> {
    let output_path = Path::new(output_path);
    let mut file = try!(File::create(output_path).map_err(|e| e.to_string()));
    try!(file.write_all(contents.as_bytes()).map_err(|e| e.to_string()));
    Ok("ok".to_string())
}


fn main() {
    // Set the base url based on an env var or use the default.
    dotenv().ok();
    let base_url = match env::var("PSEUDOC_ENDPOINT") {
        Ok(value) => value,
        Err(_) => DEFAULT_ENDPOINT.to_string(),
    };

    // Parse args.
    let args: Vec<String> = env::args().collect();
    // Get the path of the binary.
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "OUTPUT");
    opts.optopt("l", "language", "set output language", "LANGUAGE");
    opts.optflag("v", "verbose", "show more info");
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
    let verbose_mode = matches.opt_present("v");
    let input = if !matches.free.is_empty() {
        // Grab any 'free string fragments' if there are any -- this is the input FILE.
        matches.free[0].clone()
    } else {
        // If there aren't any string fragments, the program wasn't run correctly.
        print_usage(&program, opts);
        return;
    };

    // Read file contents.
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

    let code_submission_url = format!("{}/{}", &base_url, COMPILE_ROUTE);
    match post_json(&code_submission_url, &request) {
        Ok(_) => {
            if verbose_mode { println!("request sent to the cloud compiler.") }
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // Poll for result.
    let result_polling_url = format!("{}/{}/{}", &base_url, COMPILE_ROUTE, hash);
    loop {
        if verbose_mode { println!("checking the cloud compiler for a result..") }
        let compiler_response = match http_get(&result_polling_url) {
            Ok(response) => {
                match decode_response(&response) {
                    Ok(compiler_response) => compiler_response,
                    Err(err) => {
                        println!("Decoding error: {}", err);
                        return;
                    }
                }
            }
            Err(err) => {
                println!("Polling error: {}", err);
                return;
            }
        };
        if compiler_response.compilation_complete {
            // Show any errors.
            if compiler_response.error {
                if verbose_mode { println!("Compiler error:") }
                println!("{}", compiler_response.error_message);
                return;
            }
            // Save output.
            match write_file_contents(&output, &compiler_response.compiled_result) {
                Ok(_) => {
                    if verbose_mode { println!("saved result.") }
                    return
                },
                Err(err) => {
                    println!("File save error: {}", err);
                    return
                }
            }

        }

        sleep(Duration::from_secs(SECONDS_TO_SLEEP as u64));
    }
}
