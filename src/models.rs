use diesel::pg::data_types::PgTimestamp;


#[derive(Debug, Queryable)]
pub struct Submission {
	pub id: i32,

	pub submitted_at: PgTimestamp,
	pub submitted_input: String,
	pub submission_hash: String,

	pub compilation_complete: bool,
	pub compiled_at: Option<PgTimestamp>,
	pub compiled_result: Option<String>,
	pub compilation_error: Option<bool>,
	pub compilation_error_text: Option<String>,
}
