extern crate diesel;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;

extern crate pseudo;

use std::collections::HashMap;

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
//   GET  /review         shows list of submissions
//   GET  /review/<hash>  view one submission and an edit form
//   POST /review/<hash>  save work on a submission


fn main() {
	let mut server = Nickel::new();

	server.get("/", middleware! {
		"it's pseudo-lang!"
	});

	server.post("/compile", middleware! { |request|
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

	#[derive(RustcEncodable, Debug)]
	struct Sub {
		id: i32,
		submitted_contents: String,
		submitted_language: String,
	};

	server.get("/review", middleware! { |_, response|
		println!("  GET /review");
		let connection = establish_connection();
		let results = submissions.load::<Submission>(&connection)
			.expect("error loading submissions for review");
		let mut subs = Vec::new();
		for db_sub in results {
			let new_sub = Sub {
				id: db_sub.id,
				submitted_contents: db_sub.submitted_contents,
				submitted_language: db_sub.submitted_language,
			};
			subs.push(new_sub);
		}
		let mut data = HashMap::new();
		data.insert("submissions", subs);
		println!("{:?}", data);
		return response.render("assets/review-all.tpl", &data);
	});

	server.listen("127.0.0.1:6767");
}
