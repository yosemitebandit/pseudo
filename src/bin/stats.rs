extern crate diesel;
extern crate pseudo;

use self::diesel::prelude::*;
use self::pseudo::*;
use self::pseudo::models::*;


fn main() {
	use pseudo::schema::submissions::dsl::*;

	let connection = establish_connection();
	let results = submissions.filter(compilation_complete.eq(true))
		.limit(5)
		.load::<Submission>(&connection)
		.expect("error loading submissions");

	println!("displaying {} submissions", results.len());
}
