# Sita static site generator

Sita is a static site generator.

Sita aims to be simpler than other static site generators, such as Astro, Eleventy, Jekyll, Hugo, Zola, etc.


## Introduction


Sita aims to be simple for simple needs:

* Sita uses simple default settings to get you up and running.

* For example Sita can process one file from Markdown into HTML,
  without needing any special setup or any custom configuration.

* Sita can use front matter variables via HTML, JSON, TOML, YAML.


## Sita status

Sita is currently being developed:

* We're using Sita for real-world work.

* We welcome help and constructive feedback.

* You can open a GitHub issue or contact us at sita@sixarm.com.


## Sīta naming

Sīta is an earth goddess who blesses the land with good crops. In the Vedic period, she was one of the goddesses associated with fertility.

The word Sīta is a poetic term that signifies fertility and blessings coming from agriculture.


## Getting started


### --help

Get the Sita help introduction:

```
sita --help
```


### --template --input --output

Create a file `template.html` with this text:

```html
<html>
  <head>
    <title>{{ title }}</title>
  <body>
    {{{ content }}}
  </body>
</html>
```

Create a file `example.md` with this text:

```md
# lorem ipsum
dolor sit amet
```

Run Sita:

```sh
sita --template template.html --input example.md --output example.html
```

The result is the file `example.html` with this text:

```html
<html>
  <head>
    <title>lorem ipsum</title>
  <body>
    dolor sit amet
  </body>
</html>
```

What happens:

* Sita automatically sets the `{{ title }}` variable by using the input file's first headline.

* Sita automatically sets the `{{ content }}` variable by using the entire input file.

* Sita renders the template with the variables, and outputs the output file.
  

## Features

Features:

* Compile from a typical Markdown file into a typical HTML web page.

* Front matter can use HTML, JSON, TOML, YAML, or Markdown. [More…](doc/matter/)

* State variables can be set via matter or command line options. [More…](doc/state/)

* Templating uses Handlebars. More template engines in the plan. [More…](doc/templating/)

Features that we'd like to add if developers want to help or patrons want to fund: sections, pagination, taxonomies, feeds, sitemap, robots.txt, 404 error page, archive, etc.
