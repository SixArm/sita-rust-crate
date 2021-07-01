# Testing

## Testing the commands by using folders

We want to test a wide variety of commands such as:

```sh
sita --input alpha.md --output bravo.html
```

To make this simple, we use a pattern.

We create a directory for one test:

```sh
mkdir tests/alpha_bravo/
cd tests/alpha_bravo
```

We create a typical input file with Markdown:

```sh
echo "hello world" > alpha.md
```

We create a typical output file with HTML:

```sh
echo "<p>hello world</p>" > bravo.html
```

We create a corresponding output file with our naming convention of a file name suffix "=expect.html", that the tests  use to compare actual output with expected output:

```sh
echo "<p>hello world</p>" > bravo.html=expect.html
```

Our naming convention has these aspects:

* The use of "=" is a mnemonic for "this file must equal that file".

* The use of a suffix ensures the actual file and the expect file will sort together alphabetically.

* The use of the same file extension ".html" ensures that typical editors will treat the files equivalently.

