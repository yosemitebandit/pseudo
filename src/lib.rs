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
struct CompilerRequest {
	contents: String,
	language: String,
	// The hash is the MD5 of contents + language.
	hash: String,
}


#[derive(RustcDecodable)]
#[derive(Debug)]
struct CompilerResponse {
	compilation_complete: bool,
	error: bool,
	error_message: String,
	compiled_result: String,
}


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


fn http_get(url: &str) -> Result<String, String> {
	let client = Client::new();
	let mut response = client.get(url)
		.send()
		.unwrap();
	let mut body = String::new();
	response.read_to_string(&mut body).unwrap();
	Ok(body)
}


fn decode_response(response: &str) -> Result<CompilerResponse, String> {
	let response: CompilerResponse = try!(json::decode(&response).map_err(|e| e.to_string()));
	Ok(response)
}
