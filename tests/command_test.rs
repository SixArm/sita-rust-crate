use ::indoc::indoc;
use ::std::process::Command;
use ::std::path::Path;

const SITA: &str = "./target/debug/sita";

const FAB_INPUT_MARKDOWN: &str = indoc!{r#"
    # my title
    my content
"#};

const FAB_TERA_TEMPLATE: &str = indoc!{r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>{{ title }}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1">
        </head>
        <body>
    {{ content}}

        </body>
    </html>
"#};

const FAB_OUTPUT_HTML: &str = indoc!{r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>my title</title>
            <meta name="viewport" content="width=device-width, initial-scale=1">
        </head>
        <body>
    <h1>my title</h1>
    <p>my content</p>

        </body>
    </html>

"#};

fn fab_alpha() {
    ::std::fs::write("tmp/alpha.md", FAB_INPUT_MARKDOWN).expect("write");
}

fn fab_template() {
    ::std::fs::write("tmp/template.html", FAB_TERA_TEMPLATE).expect("write");
}

#[test]
fn test_1() {
    fab_alpha();
    let f = "tmp/alpha.html";
    if Path::new(f).exists() {
        ::std::fs::remove_file(f).expect("before");
    }
    assert!(!Path::new(f).exists());
    let _output = Command::new(SITA)
        .arg("tmp/alpha.md")
        .output()
        .expect("failure");
    assert!(Path::new(f).exists());
    assert_eq!(::std::fs::read_to_string(f).expect("output"), FAB_OUTPUT_HTML);
}

#[test]
fn test_arg_output() {
    fab_alpha();
    let f = "tmp/bravo.html";
    if Path::new(f).exists() {
        ::std::fs::remove_file(f).expect("before");
    }
    assert!(!Path::new(f).exists());
    let _output = Command::new(SITA)
        .arg("tmp/alpha.md")
        .arg("--output")
        .arg("tmp/bravo.html")
        .output()
        .expect("failure");
    assert!(Path::new(f).exists());
    assert_eq!(::std::fs::read_to_string(f).expect("output"), FAB_OUTPUT_HTML);
}

#[test]
fn test_arg_template() {
    fab_alpha();
    fab_template();
    let f = "tmp/alpha.html";
    if Path::new(f).exists() {
        ::std::fs::remove_file(f).expect("before");
    }
    assert!(!Path::new(f).exists());
    let _output = Command::new(SITA)
        .arg("tmp/alpha.md")
        .arg("--template")
        .arg("tmp/template.html")
        .output()
        .expect("failure");
    assert!(Path::new(f).exists());
    assert_eq!(::std::fs::read_to_string(f).expect("output"), FAB_OUTPUT_HTML);
}
