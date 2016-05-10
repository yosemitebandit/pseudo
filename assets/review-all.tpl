{{> header }}

{{#submissions}}
<p><a href="/review/{{ submission_hash }}">
	#{{ id }} -- {{ submitted_language }}
</a></p>
<textarea cols=80 rows=10 disabled>{{ submitted_contents }}</textarea>
<br />
<br />
<br />
{{/submissions}}

{{> link-bar }}
{{> footer }}
