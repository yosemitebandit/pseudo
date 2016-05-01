extern crate rustc_serialize;


#[derive(RustcEncodable)]
pub struct CompilerRequest {
	pub contents: String,
	pub language: String,
	// The hash is the MD5 of contents + language.
	pub hash: String,
}


#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct CompilerResponse {
	pub compilation_complete: bool,
	pub error: bool,
	pub error_message: String,
	pub compiled_result: String,
}
