#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rustc_serialize;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Submission, NewSubmission};


#[derive(RustcEncodable, RustcDecodable)]
#[derive(Debug)]
pub struct CompilerRequest {
	pub contents: String,
	pub language: String,
	// The hash is the MD5 of contents + language.
	pub hash: String,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct CompilerResponse {
	pub compilation_complete: bool,
	pub error: bool,
	pub error_message: String,
	pub compiled_result: String,
}


pub fn establish_connection() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("need to set DATABASE_URL in .env");
	PgConnection::establish(&database_url)
		.expect(&format!("error connecting to {}", database_url))
}


pub fn create_submission<'a>(connection: &PgConnection, contents: &'a str, language: &'a str, hash: &'a str) -> Submission {
	use schema::submissions;
	let new_submission = NewSubmission {
		submitted_contents: contents,
		submitted_language: language,
		submission_hash: hash,
	};
	diesel::insert(&new_submission).into(submissions::table)
		.get_result(connection)
		.expect("error saving new submission")
}
