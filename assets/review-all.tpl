<!doctype>
<html>
	<body>
		{{#submissions}}
		<p>
			<i>{{ id }}</i> -- {{ submitted_language }}
		</p>
		<textarea cols=80 rows=10 disabled>{{ submitted_contents }}</textarea>
		<br />
		<br />
		<br />
		{{/submissions}}
	</body>
</html>
