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
	<textarea cols=80 rows=20></textarea>
	<br />

	<input type="checkbox" id="compilation-complete">
	<label for="compilation-complete">complete</label>
	<br />

	<input type="checkbox" id="compilation-error">
	<label for="compilation-error">error</label>
	<br />
	<label for="compilation-error-message">error message:</label>
	<input id="compilation-error-message" type="text">
	<br />

	<input type="submit" value="Save">
</form>

{{> footer }}
