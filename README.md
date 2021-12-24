# Sita static site generator Rust crate

Sita is a static site generator.

Sita aims to be simple for simple needs, and flexible for complex needs.

Sita is simpler than other static site generators, such as Jekyll, Hugo, Zola, etc.

Contents:

* [](#)


## Introduction


Sita aims to be simple for simple needs:

* Sita uses simple default settings to get you up and running.

* For example Sita can process one file from Markdown into HTML,
  without needing any special setup or configuration.

Sita aims to be flexible for complex needs:

* Sita uses more capabilties to integrate your work in more ways.

* For example Sita can use front matter variables that can be set
  via HTML, or JSON, or TOML, or YAML, or other options.

Sita is currently being developed:

* We welcome help and constructive feedback.

* You can open a GitHub issue or contact us at sita@sixarm.com.


## Getting started


### --help 

Get the Sita help introduction:

```
sita --help
```


### --version

Get the Sita version number:

```
sita --version
```


### --input / -i


Create a file `example.md` with this text:

```md
hello world
```

Run Sita with an input file name:

```sh
sita --input example.md
```

The outcome is the file `example.html` with this text:

```html
<p>hello world</p>
```

The `--input` option can handle multiple files, or directories, or globs.


### --template / -t

Create a file `template.html` with this text:

```html
<html>
  <body>
    {{ content }}
  </body>
</html>
```

Run Sita with a template file name:

```sh
sita --input example.md --template template.html
```

The result is the file `example.html` with this text:

```html
<html>
  <body>
    <p>hello world</p>
  </body>
</html>
```

The `--template` option can handle multiple files, or directories, or globs.


## Features

Features:

* Compile from a typical Markdown file into a typical HTML web page.

* Front matter can be written in HTML, or JSON, or TOML, or YAML. 

Features that we may add in the future if people want to fund them:

* Sections

* Pagination

* Taxonomies

* Feeds

* Sitemap

* Robots.txt

* 404 error page

* Archive


## Template engines

Sita is being developed to use multiple template engines.

The roadmap is:

* Handlebars - because of speed and simplicity.

* Liquid - because of popularity with ecommerce developers.

* Tera - because of advanced capabilties and pure Rust.


## State variables

State variables can be set by using front matter as HTML, JSON, TOML, YAML, or command line options.


### HTML

Example:

```html
<!--
title: My Title
subtitle: My Subtitle
-->
```
```md
The content starts here.
```


### JSON

Example:

```json
{
    "title": "My Title",
    "subtitle": "My Subtitle"
}
```
```md
The content starts here.
```


### TOML

Example:

```toml
+++
title = "My Title"
subtitle = "My Subtitle"
+++
```
```md
The content starts here.
```


### YAML

Example:

```yaml
---
title: My Title
subtitle: My Subtitle
---
```
```md
The content starts here.
```


### Command line options

Sita can use front matter variables that are set using files such as:

```sh
sita example.md --variable-file var.html
sita example.md --variable-file var.json
sita example.md --variable-file var.toml
sita example.md --variable-file var.yaml
```

Sita chooses the format based on the file name.


## Issues


### TODO

TODO list in priority order:

* HTML default templates using the options `--lang`, `--title`, `--script`.

* Front matter variables via JON, TOML, YAML, and the option `--variable-file`.


## DEFERRED


### Language option

To set the language, use the command line option `--lang` such as:

```sh
$ sita example.md --lang en
```

The output file adds this HTML attribute:

```html
<html lang="en">
```


### Script option

To use a script file, use the command line option `--script` such as:

```sh
$ sita example.md --script my.js
```

The output file adds this HTML:

```html
<script src="my.js"></script>
```

You can use `--script` multiple times such as:

```sh
$ sita example.md --script graphics.js utilities.js
```
