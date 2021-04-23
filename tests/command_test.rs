use std::process::Command;
use std::path::Path;

const SITA: &str = "./target/debug/sita";

fn fab_alpha() {
    let s = indoc!{r#"
        # my title
        my content
    "#};
    ::std::fs::write("tmp/alpha.md", s).expect("write");
    assert!(Path::new("tmp/alpha.md").exists());
}

fn fab_template() {
    let s = indoc!{r#"
        <title>{{ title }}</title>
        <body>{{ content }}</body>
    "#};
    ::std::fs::write("tmp/template.html", s).expect("write");
    assert!(Path::new("tmp/template.html").exists());
}

#[test]
fn test_1() {
    fab_alpha();
    let f = "tmp/alpha.html";
    assert!(!Path::new(f).exists());
    let output = Command::new(SITA)
        .arg("tmp/alpha.md")
        .output()
        .expect("failure");
    assert!(Path::new(f).exists());
    ::std::fs::remove_file(f).expect("cleanup");
}

#[test]
fn test_arg_output() {
    fab_alpha();
    let f = "tmp/bravo.html";
    assert!(!Path::new(f).exists());
    let output = Command::new(SITA)
        .arg("tmp/alpha.md")
        .arg("--output")
        .arg("bravo.html")
        .output()
        .expect("failure");
    assert!(Path::new(f).exists());
    ::std::fs::remove_file(f).expect("cleanup");
}

#[test]
fn test_arg_template() {
    fab_alpha();
    fab_template();
    let f = "tmp/alpha.html";
    assert!(!Path::new(f).exists());
    let output = Command::new(SITA)
        .arg("tmp/alpha.md")
        .arg("--template")
        .arg("tmp/template.html")
        .output()
        .expect("failure");
    assert!(Path::new(f).exists());
    ::std::fs::remove_file(f).expect("cleanup");
}
