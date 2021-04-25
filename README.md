# Sita static site generator Rust crate

Generate a file:

```
$ sita hello.md
=> hello.html
```

To choose an ouput file use `-o` or `--output-file`:

```
$ sita hello.md -o world.html
=> world.html
```

To choose a template file use `-t` or `--template-name`:

```sh
$ sita hello.md -t default.html
=> hello.html using template file
```
