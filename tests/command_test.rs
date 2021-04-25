use ::indoc::indoc;
use ::std::process::Command;
use ::std::path::PathBuf;

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

fn fab_path(s: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tmp", s].iter().collect::<PathBuf>()
}

fn fab_input_file(path: &PathBuf) {
    ::std::fs::write(path, FAB_INPUT_MARKDOWN).expect("write");
}

fn fab_template_file(path: &PathBuf) {
    ::std::fs::write(path, FAB_TERA_TEMPLATE).expect("write");
}


#[test]
fn test_command_x_default() {
    // Prep
    let input_path = fab_path("alpha.md");
    fab_input_file(&input_path);
    let output_path = fab_path("alpha.html");
    if output_path.exists() {
        ::std::fs::remove_file(&output_path).expect("prep");
    }
    // Run
    assert!(!&output_path.exists());
    let _output = Command::new(SITA)
        .arg(&input_path)
        .output()
        .expect("failure");
    assert!(&output_path.exists());
    assert_eq!(::std::fs::read_to_string(output_path).expect("output"), FAB_OUTPUT_HTML);
}

#[test]
fn test_command_x_output_file() {
    // Prep
    let input_path = fab_path("alpha.md");
    fab_input_file(&input_path);
    let output_path = fab_path("alpha.html");
    if output_path.exists() {
        ::std::fs::remove_file(&output_path).expect("prep");
    }
    // Run
    assert!(!output_path.exists());
    let _output = Command::new(SITA)
        .arg(&input_path)
        .arg("--output-file")
        .arg(&output_path)
        .output()
        .expect("failure");
    assert!(output_path.exists());
    assert_eq!(::std::fs::read_to_string(output_path).expect("output"), FAB_OUTPUT_HTML);
}

#[test]
fn test_command_x_template_name() {
    // Prep
    let input_path = fab_path("alpha.md");
    fab_input_file(&input_path);
    let template_path = fab_path("template.html");
    fab_template_file(&template_path);
    let output_path = fab_path("alpha.html");
    if output_path.exists() {
        ::std::fs::remove_file(&output_path).expect("before");
    }
    // Run
    assert!(!output_path.exists());
    let _output = Command::new(SITA)
        .arg(&input_path)
        .arg("--template-name")
        .arg(&template_path)
        .output()
        .expect("failure");
    assert!(output_path.exists());
    assert_eq!(::std::fs::read_to_string(output_path).expect("output"), FAB_OUTPUT_HTML);
}
