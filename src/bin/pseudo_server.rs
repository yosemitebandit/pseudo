extern crate diesel;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;

extern crate pseudo;

use std::collections::HashMap;

use nickel::{Nickel, HttpRouter, JsonBody, FormBody};
use nickel::extensions::{Redirect};
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

	server.get("/", middleware! { |_, response|
		let mut data = HashMap::new();
		data.insert("submissions", "");
		return response.render("assets/index.tpl", &data);
	});

	server.post("/compile", middleware! { |request|
		let compiler_request = request.json_as::<CompilerRequest>().unwrap();
		//println!("received a submission with hash: {}", compiler_request.hash);
		// See if we already have that hash in the DB.
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(&compiler_request.hash))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash in post");
		// If there are no results, create a new submission in the DB.
		if results.len() == 0 {
			let submission = create_submission(&connection, &compiler_request.contents, &compiler_request.language, &compiler_request.hash);
			//println!("saved submission with id {}.", submission.id);
		} else {
			//println!("already have submission with hash {}", &compiler_request.hash)
		}
		"ok"
	});

	server.get("/compile/:hash", middleware! { |request|
		//println!("  GET for hash: {:?}", request.param("hash"));
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(request.param("hash").unwrap()))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash");

		if results.len() == 1 {
			let submission = results[0].clone();
			//println!("{:?}", submission);
			let compiler_response = CompilerResponse {
				compilation_complete: submission.compilation_complete,
				compiled_result: submission.compiled_result.unwrap_or("".to_string()),
				error: submission.compilation_error.unwrap_or(false),
				error_message: submission.compilation_error_message.unwrap_or("".to_string()),
			};
			let encoded = json::encode(&compiler_response).unwrap();
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
		submission_hash: String,
		compilation_complete: bool,
		compiled_result: String,
		compilation_error: bool,
		compilation_error_message: String,
	};

	server.get("/review", middleware! { |_, response|
		//println!("  GET /review");
		let connection = establish_connection();
		let results = submissions.load::<Submission>(&connection)
			.expect("error loading submissions for review");
		let mut subs = Vec::new();
		for db_sub in results {
			let new_sub = Sub {
				id: db_sub.id,
				submitted_contents: db_sub.submitted_contents,
				submitted_language: db_sub.submitted_language,
				submission_hash: db_sub.submission_hash,
				compilation_complete: db_sub.compilation_complete,
				compiled_result: db_sub.compiled_result.unwrap_or("".to_string()),
				compilation_error: db_sub.compilation_error.unwrap_or(false),
				compilation_error_message: db_sub.compilation_error_message.unwrap_or("".to_string()),
			};
			subs.push(new_sub);
		}
		let mut data = HashMap::new();
		data.insert("submissions", subs);
		return response.render("assets/review-all.tpl", &data);
	});

	server.get("/review/:hash", middleware! { |request, response|
		//println!("  GET /review/{}", request.param("hash").unwrap());
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(request.param("hash").unwrap()))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash");
		if results.len() == 1 {
			let db_sub = results[0].clone();
			let sub = Sub {
				id: db_sub.id,
				submitted_contents: db_sub.submitted_contents,
				submitted_language: db_sub.submitted_language,
				submission_hash: db_sub.submission_hash,
				compilation_complete: db_sub.compilation_complete,
				compiled_result: db_sub.compiled_result.unwrap_or("".to_string()),
				compilation_error: db_sub.compilation_error.unwrap_or(false),
				compilation_error_message: db_sub.compilation_error_message.unwrap_or("".to_string()),
			};
			let mut data = HashMap::new();
			data.insert("submission", sub);
			return response.render("assets/review-single.tpl", &data);
		} else if results.len() == 0 {
			"404"
		} else {
			"500"
		}
	});

	server.post("/review/:hash", middleware! { |request, response|
		let hash = request.param("hash").unwrap().to_owned();

		let form_data = try_with!(response, request.form_body());
		//println!("{:?}", form_data);
		let mut complete_box = false;
		if form_data.get("compilation-complete").unwrap_or("off") == "on" {
			complete_box = true;
		}
		let mut error_box = false;
		if form_data.get("compilation-error").unwrap_or("off") == "on" {
			error_box = true;
		}
		let error_message = form_data.get("compilation-error-message").unwrap_or("Error");
		let result = form_data.get("compiled-result").unwrap_or("");

		// Update the submission in the DB..struggling with updating multiple fields.
		let connection = establish_connection();
		diesel::update(submissions.filter(submission_hash.eq(&hash)))
			.set(compilation_complete.eq(complete_box))
			.execute(&connection);
		diesel::update(submissions.filter(submission_hash.eq(&hash)))
			.set(compiled_result.eq(result))
			.execute(&connection);
		diesel::update(submissions.filter(submission_hash.eq(&hash)))
			.set(compilation_error.eq(error_box))
			.execute(&connection);
		diesel::update(submissions.filter(submission_hash.eq(&hash)))
			.set(compilation_error_message.eq(error_message))
			.execute(&connection);

		return response.redirect(format!("/review/{}", hash));
	});

	server.listen("127.0.0.1:6767");
}
