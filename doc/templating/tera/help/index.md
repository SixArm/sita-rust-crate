# Templating Help for Tera

A Tera Rust template is a text file syntax is based on Jinja2/Django templates. When the app renders a template, then the template's variables and expressions get replaced with values.

Syntax:

```tera
{# … #} is a comment
{% … %} is a statement
{{ … }} is an expression
```

Example:

```tera
{# This is a comment #}
{% set x = 1 %}
{{ x }}
```


## If then else

If:

```tera
{% if x %}
    Hello
{% endif %}
```

Else:

```tera
{% if x %}
    Hello
{% else %}
    World
{% endif %}
```


### Functions

Function examples (and see [more](https://tera.netlify.app/docs/#built-in-functions))


```
get_env(name, default) // Return the environment variable value for the name.
get_random(start, end) // Returns a random integer in the given range.
now(timestamp, utc) // Return the current datetime.
range(end, start, step_by) // Return an array of integers.
throw(message) // Show a template rendering error message.
```
