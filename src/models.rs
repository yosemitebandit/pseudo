use diesel::pg::data_types::PgTimestamp;


#[derive(Clone, Debug, Queryable)]
pub struct Submission {
	pub id: i32,

	pub submitted_at: PgTimestamp,
	pub submitted_contents: String,
	pub submitted_language: String,
	pub submission_hash: String,

	pub compilation_complete: bool,
	pub compiled_at: Option<PgTimestamp>,
	pub compiled_result: Option<String>,
	pub compilation_error: Option<bool>,
	pub compilation_error_message: Option<String>,
}


use super::schema::submissions;

#[insertable_into(submissions)]
pub struct NewSubmission<'a> {
	pub submitted_contents: &'a str,
	pub submitted_language: &'a str,
	pub submission_hash: &'a str,
}
