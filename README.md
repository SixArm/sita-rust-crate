# Sita static site generator Rust crate

Sita is a static site generator that aims to be simple for simple needs, and flexible for complex needs. Sita is similar in ways to Jekyll, Hugo, Zola, and other static site generators.

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

Sita is currently being developed and we welcome help.

We especially thank the authors of Zola and Tera.


## Examples


### Beginner commands

The beginner commands are:

```
$ sita --help
$ sita --version
```


### Hello world

Create a file `example.md` with this text:

```
hello world
```

To generate a page:

```
$ sita example.md
```

The outcome is a new page file `example.html` with this text:

```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>hello world</title>
    <head>
    <body>
        hello world
    </body>
</html>
```


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


### Template file option

First create a file `template.html` with any HTML such as:

```html
<b>{{ content }}</b>
```

To use a template file, use the command line option `-t` or `--template-name`:

```sh
$ sita example.md --template-file template.html
```

The outcome is a new page file `hello.html` rendered with the template file `template.html`:

```html
<b>hello world</b>
```

You can use `--template-file` multiple times; the template files are concatenated.


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
