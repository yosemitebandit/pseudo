extern crate diesel;
extern crate dotenv;
extern crate getopts;
#[macro_use] extern crate nickel;
extern crate rand;
extern crate rustc_serialize;

extern crate pseudo;

use std::collections::HashMap;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use dotenv::dotenv;
use getopts::Options;
use nickel::{Nickel, HttpRouter, JsonBody, FormBody};
use nickel::extensions::{Redirect};
use rand::Rng;
use rustc_serialize::json;

use self::pseudo::*;
use self::pseudo::models::*;
use self::pseudo::schema::submissions::dsl::*;
use self::diesel::prelude::*;


const DEFAULT_SECRET_TOKEN: &'static str = "top secret!";
const DEFAULT_HOST_AND_PORT: &'static str = "127.0.0.1:5000";


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
	// Load the .env file, looking for the secret token.
	dotenv().ok();
	let secret_token = match env::var("SECRET_TOKEN") {
		Ok(value) => value,
		Err(_) => DEFAULT_SECRET_TOKEN.to_string(),
	};

	// Get the command line args.
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let mut opts = Options::new();
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
		return
	}
	let verbose_mode = matches.opt_present("v");

	// Setup the server and routes.
	let mut server = Nickel::new();

	server.get("/", middleware! { |_, response|
		if verbose_mode { println!("GET /") }
		let mut data = HashMap::new();
		data.insert("placeholder", "");
		return response.render("assets/index.tpl", &data);
	});

	server.post("/compile", middleware! { |request|
		let compiler_request = request.json_as::<CompilerRequest>().unwrap();
		if verbose_mode { println!("POST to /compile with hash: {}", compiler_request.hash) }
		// See if we already have that hash in the DB.
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(&compiler_request.hash))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash in post");
		// If there are no results, create a new submission in the DB.
		if results.len() == 0 {
			let submission = create_submission(&connection, &compiler_request.contents, &compiler_request.language, &compiler_request.hash);
			if verbose_mode { println!("saved submission with id {}.", submission.id) }
		} else {
			if verbose_mode { println!("already have submission with hash {}", &compiler_request.hash) }
		}
		"ok"
	});

	server.get("/compile/:hash", middleware! { |request|
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(request.param("hash").unwrap()))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash");

		if results.len() == 1 {
			if verbose_mode { println!("GET /compile/{:?}", request.param("hash")) }
			let submission = results[0].clone();
			let compiler_response = CompilerResponse {
				compilation_complete: submission.compilation_complete,
				compiled_result: submission.compiled_result.unwrap_or("".to_string()),
				error: submission.compilation_error.unwrap_or(false),
				error_message: submission.compilation_error_message.unwrap_or("".to_string()),
			};
			let encoded = json::encode(&compiler_response).unwrap();
			encoded
		} else if results.len() == 0 {
			if verbose_mode { println!("GET /compile/{:?} -> 404", request.param("hash")) }
			"404".to_string()
		} else {
			if verbose_mode { println!("GET /compile/{:?} -> 500", request.param("hash")) }
			"500".to_string()
		}
	});

	#[derive(RustcEncodable, Debug)]
	struct SubmissionFormData {
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
		if verbose_mode { println!("GET /review") }
		let connection = establish_connection();
		let results = submissions.load::<Submission>(&connection)
			.expect("error loading submissions for review");
		let mut subs = Vec::new();
		for db_sub in results {
			let new_sub = SubmissionFormData {
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
		let connection = establish_connection();
		let results = submissions.filter(submission_hash.eq(request.param("hash").unwrap()))
			.load::<Submission>(&connection)
			.expect("error loading submissions matching hash");
		if results.len() == 1 {
			if verbose_mode { println!("GET /review/{}", request.param("hash").unwrap()) }
			let db_sub = results[0].clone();
			let sub = SubmissionFormData {
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
			if verbose_mode { println!("GET /review/{} => 404", request.param("hash").unwrap()) }
			"404"
		} else {
			if verbose_mode { println!("GET /review/{} => 500", request.param("hash").unwrap()) }
			"500"
		}
	});

	server.post("/review/:hash", middleware! { |request, response|
		let hash = request.param("hash").unwrap().to_owned();

		let form_data = try_with!(response, request.form_body());
		let token = form_data.get("token").unwrap_or("");
		let seconds_to_sleep = rand::thread_rng().gen_range(1f64, 5f64);
		sleep(Duration::from_secs(seconds_to_sleep as u64));
		if token != secret_token {
			if verbose_mode { println!("unauthorized edit attempted for hash {}", hash) }
			return response.redirect(format!("/review/{}", hash));
		}
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
		if verbose_mode { println!("POST /review/{}", hash) }

		// Update the submission in the DB.
		let connection = establish_connection();
		diesel::update(submissions.filter(submission_hash.eq(&hash)))
			.set((compilation_complete.eq(complete_box), compiled_result.eq(result), compilation_error.eq(error_box), compilation_error_message.eq(error_message)))
			.execute(&connection);
		return response.redirect(format!("/review/{}", hash));
	});

	let host_and_port = match env::var("PSEUDO_SERVER_HOST_AND_PORT") {
		Ok(value) => value.to_string(),
		Err(_) => DEFAULT_HOST_AND_PORT.to_string(),
	};
	server.listen(&host_and_port[..]);
}
