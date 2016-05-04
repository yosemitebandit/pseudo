CREATE TABLE submissions (
  id SERIAL PRIMARY KEY,

  submitted_at TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  submitted_input TEXT NOT NULL,
  submission_hash TEXT NOT NULL,

  compilation_complete BOOLEAN NOT NULL DEFAULT 'f',
  compiled_at TIMESTAMP,
  compiled_result TEXT,
  compilation_error BOOLEAN,
  compilation_error_text TEXT
)
