# Sita static site generator Rust crate

Generate one static site file:

```
$ sita hello.md
=> hello.html
```

Choose a template file with `-t` or `--template`:

```sh
$ sita hello.md -t typical.html
=> hello.html (with template file typical.html)
```

Choose an ouput file with `-o` or `--output`:

```
$ sita hello.md -o world.html
=> world.html
```
