# Sita static site generator Rust crate

Sita is a static site generator. Sita aims to be simple for simple needs, and flexible for complex needs.  Sita is similar in ways to Jekyll, Hugo, Zola, and other static site generators.

Contents:

* [](#)


## Introduction


Sita aims to be simple for simple needs, and flexible for complex needs. 

* Sita is simple for simple needs. For example Sita can process one text file
  from Markdown to HTML, without needing any special directories,
  configurations, templates, variables, and so on. This is because Sita aims to
  use defaults for all settingqs.

* Sita is flexible for complex needs. For example Sita can use front matter
  variables that can be set via HTML, JSON, TOML, YAML, or command line options
  that specify varibles as files. This is because Sita aims to work with a range
  of formats.

Sita is currently being developed. We welcome help.

We especially thank the authors of Zola and Tera.


## Examples


### Beginner commands

The beginner commands are:

```
$ sita --help
$ sita --version
```


### Create hello.md

Create a file `hello.md` with this text:

```md
hello world
```


### -i --input

To run Sita with an input file, use the command line option `-i` or `--input`:

```sh
sita --input hello.md
```

The output is a file `hello.html` with this text:

```html
<p>hello world</p>
```


### -o --output

To run Sita with an output file, use the command line option `-o` or `--output`:

```sh
sita --input hello.md --output world.html
```

The output is a file `world.html` with this text:

```html
<p>hello world</p>
```


### --io (TODO)

To run Sita with an input file and output file, ue the command line option `--io` or `--input-output`:

```sh
sita --io hello.md world.html
```

The output is a file `world.html` with this text:

```html
<p>hello world</p>
```


### Create template.html

Create a file `template.html` with this text:

```html
<html><body>{{ content }}</body></html>
```


### -t --template

To run Sita with a template file, use the command line option `-t` or `--template`:

```sh
sita --input hello.md --template template.html
```

The output is the file `hello.html` and it is now rendered with the template file `template.html`:

```html
<html><body><p>hello world</p></body></html>
```


### Output file option

To choose an ouput file use the command line option `-o` or `--output-file`:

```
$ sita example.md --output-file output.html
```

The outcome is a new page file `output.html`.

The output file name can be:

  * A base name such as `output.html`

  * A relative path such as `build/output.html` 

  * An absolute path such as `/tmp/output.html`


## Front matter variables

Front matter variables can be set by using HTML, JSON, TOML, YAML, or command line options.


### HTML

Example:

```md
<!--
title: My Title
subtitle: My Subtitle
-->
The content starts here.
```


### JSON

```md
{
    "title": "My Title",
    "subtitle": "My Subtitle"
}
The content starts here.
```


### TOML

```md
+++
title = "My Title"
subtitle = "My Subtitle"
+++
The content starts here.
```


### YAML

```md
---
title: My Title
subtitle: My Subtitle
---
The content starts here.
```


### Command line options

Sita can use front matter variables that are set using files such as:

```sh
$ sita example.md --variable-file var.html
$ sita example.md --variable-file var.json
$ sita example.md --variable-file var.toml
$ sita example.md --variable-file var.yaml
```

Sita chooses the format based on the file name.


## Issues


### TODO

TODO list in priority order:

* HTML default templates using the options `--lang`, `--title`, `--script`, `--stylesheet`.

* Front matter variables via JON, TOML, YAML, and the option `--variable-file`.


### UTF-8

The command line options for `--script` and `--stylesheet` require UTF-8 strings in order to create valid URLs. 

* These options cannot currently use OS-specific non-UTF-8 file names.

* This means the options cannot point to a non-UTF-8 URL, such as a local file name that uses ASCII-only encoding.


## DEFERRED


### Title option

To set the title, use the command line option `--title` such as:

```sh
$ sita example.md --title Welcome
```

The output file adds this HTML:

```html
<title>Welcome</title>
```


### Language option

To set the language, use the command line option `--lang` such as:

```sh
$ sita example.md --lang en
```

The output file adds this HTML attribute:

```html
<html lang="en">
```


### Stylesheet option

To use a stylesheet file, use the command line option `--stylesheet` such as:

```sh
$ sita example.md --stylesheet my.css
```

The output file adds this HTML:

```html
<link rel="stylesheet" href="my.css">
```

You can use `--stylesheet` multiple times such as:

```sh
$ sita example.md --stylesheet reset.css --stylesheet screen.js
```


### Script option

To use a script file, use the command line option `--script` such as:

```sh
$ sita example.md --script-file my.js
```

The output file adds this HTML:

```html
<script src="my.js"></script>
```

You can use `--script` multiple times such as:

```sh
$ sita example.md --script graphics.js --script utilities.js
```
