#[macro_use] extern crate nickel;
extern crate rustc_serialize;

extern crate pseudo;

use nickel::{Nickel, HttpRouter, JsonBody};
use rustc_serialize::json;

use self::pseudo::*;


fn main() {
	let mut server = Nickel::new();

	server.post("/", middleware! { |request, response|
		let compiler_request = request.json_as::<CompilerRequest>().unwrap();
		println!("received a new submission with hash: {:?}", compiler_request.hash);
		let connection = establish_connection();
		let submission = create_submission(&connection, &compiler_request.contents, &compiler_request.language, &compiler_request.hash);
		println!("saved submission with id {}.", submission.id);
		"ok"
	});

	server.get("/:hash", middleware! { |request|
		println!("  GET for hash: {:?}", request.param("hash"));
		let response = CompilerResponse {
			compilation_complete: true,
			error: false,
			error_message: "".to_string(),
			compiled_result: "while(True)\nprint 'hey there!'".to_string(),
		};
		let encoded = json::encode(&response).unwrap();
		encoded
	});

	server.listen("127.0.0.1:6767");
}
