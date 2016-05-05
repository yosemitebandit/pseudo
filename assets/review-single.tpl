<!doctype>
<html>
	<body>
		<p>#{{ submission.id }} -- {{ submission.submitted_language }}</p>
		<textarea cols=80 rows=20 disabled>{{ submission.submitted_contents }}</textarea>
		<br />
		<br />
		<br />
	</body>
</html>
