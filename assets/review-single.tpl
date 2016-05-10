{{> header }}
{{> top-bar }}

<p>#{{ submission.id }} -- {{ submission.submitted_language }}</p>
<textarea cols=80 rows=20 disabled>{{ submission.submitted_contents }}</textarea>

<br />
<br />
<br />

<form action="" method="post">
	<label for="compiled-result">compiled result:</label>
	<br />
	<textarea cols=80 rows=20 name="compiled-result">{{ submission.compiled_result }}</textarea>
	<br />

	<input type="checkbox" name="compilation-complete"
	{{#submission.compilation_complete}}
	checked
	{{/submission.compilation_complete}}
	>
	<label for="compilation-complete">complete</label>
	<br />

	<input type="checkbox" name="compilation-error"
	{{#submission.compilation_error}}
	checked
	{{/submission.compilation_error}}
	>
	<label for="compilation-error">error</label>
	<br />

	<label for="compilation-error-message">error message:</label>
	<input type="text" name="compilation-error-message" value="{{ submission.compilation_error_message }}" size=80>
	<br />

	<label for="token">token:</label>
	<input type="password" name="token">
	<br />

	<input type="submit" formenctype="application/x-www-form-urlencoded" value="Save">
</form>

{{> footer }}
