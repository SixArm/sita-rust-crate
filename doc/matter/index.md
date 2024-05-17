# Matter

Matter is our generic term for web page file front matter.


## Flavors

Various blog tools can use front matter with various flavors, meaning various formats such as HTML, or JSON, or TOML, or YAML.

Sita can use any of these flavors.


### HTML

Example of front matter with HTML:

```
<!--
title: Hello World
contact: alice@example.com
-->
Content starts here.
```


### JSON

Example of front matter with JSON:

```
{
    "title": "Hello World",
    "contact": "alice@example.com"
}
Content starts here.
```


### TOML

Example of front matter with TOML:

```
+++
title = "Hello World"
contact = "alice@example.com"
+++
Content starts here.
```


### YAML

Example of front matter with YAML:

```
---
title: Hello World
contact: alice@example.com
---
Content starts here.
```

### Markdown comments

Example of front matter with Markdown comments:

```
[//] # (title: Hello World)
[//] # (contact: alice@example.com)

Content starts here.
```
