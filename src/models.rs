use diesel::pg::data_types::PgTimestamp;


#[derive(Queryable)]
pub struct Submission {
	pub id: i32,
	pub submitted_at: PgTimestamp,
	pub submitted_input: String,
	pub complete: bool,
	pub compiled_at: PgTimestamp,
	pub compiled_result: String,
	pub error: bool,
	pub error_text: String,
}
