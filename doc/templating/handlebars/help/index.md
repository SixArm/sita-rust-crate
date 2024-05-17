# Templating help for Handlebars

A Handlebars Rust template is a text file syntax is based on JavaScript Handlebars templates. When the app renders a template, then the template's variables and expressions get replaced with values.

Syntax:

* `{{…}}` expression that is HTML-escaped by default.
* `{{{…}}}` expression that is not HTML-escaped by default.
* `{{! …}}` comment that is omitted from output.
* `{{{{raw}}}}…{{{{/raw}}}}` escape handlebars expression within the block
* `{{#if …}} … {{else}} … {{/if}}` if-else block
* `{{#unless …}} … {{else}} .. {{/unless}}` if-not-else block
* `{{#each …}} … {{/each}}` iterate over an array or object. Note that Handlebars-Rust doesn't support mustache iteration syntax so use this instead.
* `{{#with …}} … {{/with}}` change current context. Similar to `{{#each}}` and used to replace corresponding mustache syntax.
* `{{lookup … …}}` get value from array by @index or @key
* `{{> …}}` include template with name
* `{{log …}}` log value with rust logger, default level: INFO. Currently you cannot change the level.

Boolean helpers that can be used in if as subexpression, for example `{{#if (gt 2 1)}} …`:

* `eq`
* `ne`
* `gt`
* `gte`
* `lt`
* `lte`
* `and`
* `or`
* `not`

Example:

```handlebars
{# This is a comment #}
{% set x = 1 %}
{{ x }}
```


## If then else

If:

```handlebars
{% if x %}
    Hello
{% endif %}
```

Else:

```handlebars
{% if x %}
    Hello
{% else %}
    World
{% endif %}
```


### Functions

Function examples (and see [more](https://handlebars.netlify.app/docs/#built-in-functions))


```
get_env(name, default) // Return the environment variable value for the name.
get_random(start, end) // Returns a random integer in the given range.
now(timestamp, utc) // Return the current datetime.
range(end, start, step_by) // Return an array of integers.
throw(message) // Show a template rendering error message.
```
