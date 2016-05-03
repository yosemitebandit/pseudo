CREATE TABLE submissions (
  id SERIAL PRIMARY KEY,
  submitted_at TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  submitted_input TEXT NOT NULL,
  complete BOOLEAN NOT NULL DEFAULT 'f',
  compiled_at TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  compiled_result TEXT NOT NULL,
  error BOOLEAN NOT NULL DEFAULT 'f',
  error_text TEXT NOT NULL
)
