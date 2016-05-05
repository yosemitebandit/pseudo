extern crate diesel;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;

extern crate pseudo;

use nickel::{Nickel, HttpRouter, JsonBody};
use rustc_serialize::json;

use self::pseudo::*;
use self::pseudo::models::*;
use self::pseudo::schema::submissions::dsl::*;
use self::diesel::prelude::*;


// routes:
//   GET  /               shows some info
//   POST /               405
//   GET  /compile        405
//   POST /compile        creates a new submission -> JSON
//   GET  /compile/<hash> gets info on a submission -> JSON or 404
//   POST /compile/<hash> 405


fn main() {
	let mut server = Nickel::new();

	server.get("/", middleware! { |request, response|
		"it's pseudo-lang!"
	});

	server.post("/compile", middleware! { |request, response|
		let request = request.json_as::<CompilerRequest>().unwrap();
		println!("received a submission with hash: {}", request.hash);
		// See if we already have that hash in the DB.
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(&request.hash))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash in post");
		// If there are no results, create a new submission in the DB.
		if results.len() == 0 {
			let submission = create_submission(&connection, &request.contents, &request.language, &request.hash);
			println!("saved submission with id {}.", submission.id);
		} else {
			println!("already have submission with hash {}", &request.hash)
		}
		"ok"
	});

	server.get("/compile/:hash", middleware! { |request|
		println!("  GET for hash: {:?}", request.param("hash"));
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(request.param("hash").unwrap()))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash");

		if results.len() == 1 {
			let submission = results[0].clone();
			let response = CompilerResponse {
				compilation_complete: submission.compilation_complete,
				compiled_result: submission.compiled_result.unwrap_or("".to_string()),
				error: submission.compilation_error.unwrap_or(false),
				error_message: submission.compilation_error_message.unwrap_or("".to_string()),
			};
			let encoded = json::encode(&response).unwrap();
			encoded
		} else if results.len() == 0 {
			"404".to_string()
		} else {
			"500".to_string()
		}

	});

	server.listen("127.0.0.1:6767");
}
