//! Command line argument parsing (CLAP) for the application.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approachs:
//!
//!   * via typical functions, which favors advanced uses yet is verbose.
//!   * via usage strings, which looks more like writing documentation.
//!   * via macros, which is fast and less verbose, yet atypical to read.
//!   * via YAML file, which favors localization and text file readability.
//!
//! We prefer the typical functions, because they provide maximum capability,
//! and in our experience are the easiest for Rust IDEs to read and debug.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we like the separation of concerns.


use clap::{Arg, Command};
use crate::app::args::Args;
use crate::types::*;
use crate::test::*;

/// Create a clap app.
pub fn app() -> Command<'static> {
    trace!("clap::app");
    Command::new("Sita")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .alias("inputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("An input path string.\nExample file: --input \"example.html\" …\nExample directory: --input \"examples/\" …\nExample glob: --input \"examples/**/*\" …"))
    // .arg(Arg::new("input_file_name_select_regex_string")
    //     .long("input-select")
    //     .value_name("REGEX")
    //     .takes_value(true)
    //     .help("The input file name select filter regular expression.\nExample: --input-select: \"^foo\" (starts with \"foo\")\nExample: --input-select \"md$\" (ends with \"md\")"))
    .arg(Arg::new("output")
        .short('o')
        .long("output")
        .alias("outputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("An output path string.\nExample file: --output \"example.html\" …\nExample directory: --output \"examples/\" …\nExample glob: --output \"examples/**/*\" …"))
    .arg(Arg::new("output_file_name_extension")
        .long("output-extension")
        .value_name("EXTENSION")
        .takes_value(true)
        .help("The output file name extension.\nDefault: \"html\".\nExample: --output-extension \"html\""))
    .arg(Arg::new("template")
        .short('t')
        .long("template")
        .alias("templates")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("A template path string.\nExample file: --template \"example.html\" …\nExample directory: --template \"examples/\" …\nExample glob: --template \"examples/**/*\" …"))
    .arg(Arg::new("helper")
        .short('h')
        .long("helper")
        .alias("helpers")
        .value_name("FILE | DIRECTORY | GLOB")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .help("A helper path string.\nExample file: --helper \"example.rhai\" …\nExample directory: --helper \"helpers/\" …\nExample glob: --helper \"helpers/**/*\" …"))
    .arg(Arg::new("test")
        .long("test")
        .takes_value(false)
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test"))
    // .arg(Arg::new("set")
    //     .short('s')
    //     .long("set")
    //     .value_names(&["NAME", "VALUE"])
    //     .takes_value(true)
    //     .multiple_occurrences(true)
    //     .multiple_values(true)
    //     .help("Set a variable name to a value.\nExample: --let pi \"3.1415\" …"))
    // .arg(Arg::new("verbose")
    //     .short('v')
    //     .long("verbose")
    //     .takes_value(false)
    //     .multiple_occurrences(true)
    //     .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …"))
}

/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    trace!("clap::args");
    let matches = app().get_matches();
    trace!("clap::args matches: {:?}", matches);

    let input_list_pathable_string = match matches.values_of("input") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    // let input_file_name_select_regex_string = match matches.value_of("input_file_name_select_regex_string") {
    //     Some(x) => Some(String::from(x)),
    //     _ => None,
    // };

    let output_list_pathable_string = match matches.values_of("output") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let output_file_name_extension = match matches.value_of("output_file_name_extension") {
        Some(x) => Some(String::from(x)),
        _ => None,
    };

    // let helper_url_list = match matches.values_of("helper") {
    //     Some(x) => Some(x.map(|x| String::from(x)).collect::<List<UrlString>>()),
    //     _ => None,
    // };

    // let settings = match matches.values_of("set") {
    //     Some(x) => {
    //         let vec: Vec<&str> = x.collect();
    //         Some(from_list_str_into_map_string_string(&vec))
    //     },
    //     _ => None,
    // };

    // let log_level = match matches.occurrences_of("verbose") {
    //     0 => None,
    //     1 => Some(::log::Level::Error),
    //     2 => Some(::log::Level::Warn),
    //     3 => Some(::log::Level::Info),
    //     4 => Some(::log::Level::Debug),
    //     5 => Some(::log::Level::Trace),
    //     _ => Some(::log::Level::Trace),
    // };

    let template_list_pathable_string = match matches.values_of("template") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let helper_list_pathable_string = match matches.values_of("helper") {
        Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
        _ => None,
    };

    let test = matches.is_present("test");

    let args = Args {
        input_list_pathable_string: input_list_pathable_string,
        //input_file_name_select_regex_string: input_file_name_select_regex_string,
        output_list_pathable_string: output_list_pathable_string,
        output_file_name_extension: output_file_name_extension,
        // settings: settings,
        template_list_pathable_string: template_list_pathable_string,
        helper_list_pathable_string: helper_list_pathable_string,
        test: test,
        // log_level: log_level,
    };

    trace!("clap::args -> {:?}", args);
    args    
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that the special argument `--test` is working.
    //
    // This test must succeed in order for any of the rest of the tests here to
    // succeed, because the `--test` argument turns on the runtime output to stdout,
    // which includes a typical print debug of the entire `args` structure.
    //
    #[test]
    fn test_test() {
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test"]);
        let target = r#"Args { "#;
        assert_command_stdout_contains!(command, &target);
    }

    // Test that the special argument `--verbose` is working.
    //
    // This test must succeed in order for any of the rest of the tests here to
    // show diagnostics, because the `--verbose` argument turns on logging output,
    // which can include debugging messages, warnings, errors, and so on.
    //
    // #[test]
    // fn test_verbose() {
    //     assert_command_stdout_contains!(COMMAND, &["--test"], r#" log_level: None"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "-v"], r#" log_level: Some(Error)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "-vv"], r#" log_level: Some(Warn)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "-vvv"], r#" log_level: Some(Info)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "-vvvv"], r#" log_level: Some(Debug)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "-vvvvv"], r#" log_level: Some(Trace)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "--verbose"], r#" log_level: Some(Error)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "--verbose", "--verbose"], r#" log_level: Some(Warn)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Info)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Debug)"#);
    //     assert_command_stdout_contains!(COMMAND, &["--test", "--verbose", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Trace)"#);
    // }

    // #[path = "util.rs"]
    // mod util;
    // use util::*;


    #[test]
    fn test_input() {
        let s1 = "alpha";
        let s2 = "bravo";
        let s3 = "charlie";
        let s4 = "delta";
        let target = format!(" input_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

        // Test short `-i` with multiple occurances and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-i", &s1, &s2, "-i", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--input` with multiple occurances and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--input", &s1, &s2, "--input", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--inputs` with multiple occurances and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--inputs", &s1, &s2, "--inputs", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);
    }

    #[test]
    fn test_output() {
        let s1 = "alpha";
        let s2 = "bravo";
        let s3 = "charlie";
        let s4 = "delta";
        let target = format!(" output_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

        // Test short `-o` with multiple occurrences and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-o", &s1, &s2, "-o", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--output` with multiple occurrences and multiple values
        // Test short `-o` with multiple occurrences and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--output", &s1, &s2, "--output", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--outputs` with multiple occurrences and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--outputs", &s1, &s2, "--outputs", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);
    }

    #[test]
    fn test_clap_output_file_name_extension() {
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--output-extension", "alpha"]);
        let target = r#" output_file_name_extension: Some("alpha")"#;
        assert_command_stdout_contains!(command, &target);
    }

    #[test]
    fn test_template() {
        let glob_dir = "template_list_pathable_string";
        let glob1 = format!("{}/{}", &glob_dir, "a/**/*");
        let glob2 = format!("{}/{}", &glob_dir, "b/**/*");
        let glob3 = format!("{}/{}", &glob_dir, "c/**/*");
        let glob4 = format!("{}/{}", &glob_dir, "d/**/*");
        let target = format!(" template_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &glob1, &glob2, &glob3, &glob4);

        // Test short `-t` with multiple occurances and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-t", &glob1, &glob2, "-t", &glob3, &glob4]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--template` with multiple occurances and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--template", &glob1, &glob2, "--template", &glob3, &glob4]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--templates` with multiple occurances and multiple values
        let mut command = ::std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--templates", &glob1, &glob2, "--templates", &glob3, &glob4]);
        assert_command_stdout_contains!(command, &target);

    }

    // #[test]
    // fn test_clap_template_glob_to_template_path_set() {
    //     let dir = "from_set_pathable_string_into_set_path_buf/";
    //     assert_command_stdout_contains!(
    //         COMMAND, 
    //         &[
    //             "--test", 
    //             "--template", 
    //             &format!("{}{}", &dir, "a/**/*"), 
    //             &format!("{}{}", &dir, "b/**/*"), 
    //             "--template", 
    //             &format!("{}{}", &dir, "c/**/*"), 
    //             &format!("{}{}", &dir, "d/**/*"), 
    //         ], 
    //         &format!(" template_path_set:  Some([\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\"]",
    //             &format!("{}{}", &dir, "a/aa"),
    //             &format!("{}{}", &dir, "a/aa/aaa"),
    //             &format!("{}{}", &dir, "a/aa/aab"),
    //             &format!("{}{}", &dir, "a/ab"),
    //             &format!("{}{}", &dir, "a/ab/aba"),
    //             &format!("{}{}", &dir, "a/ab/abb"),
    //             &format!("{}{}", &dir, "b/ba"),
    //             &format!("{}{}", &dir, "b/ba/baa"),
    //             &format!("{}{}", &dir, "b/ba/bab"),
    //             &format!("{}{}", &dir, "b/bb"),
    //             &format!("{}{}", &dir, "b/bb/bba"),
    //             &format!("{}{}", &dir, "b/bb/bbb"),
    //             &format!("{}{}", &dir, "b/bb/bbb"),
    //             &format!("{}{}", &dir, "c/ca/caa"),
    //             &format!("{}{}", &dir, "c/ca/cab"),
    //             &format!("{}{}", &dir, "c/cb/cba"),
    //             &format!("{}{}", &dir, "c/cb/cbb"),
    //             &format!("{}{}", &dir, "d/da/daa"),
    //             &format!("{}{}", &dir, "c/da/dab"),
    //             &format!("{}{}", &dir, "d/db/dba"),
    //             &format!("{}{}", &dir, "d/db/dbb"),
    //         )
    //     );
    // }

    // #[test]
    // fn test_set() {
    //     let mut command = ::std::process::Command::new(&*COMMAND_OS);
    //     command.args(&["--test", "--set", "alpha", "bravo", "--set", "charlie", "delta"]);
    //     let target = r#" settings: Some({"alpha": "bravo", "charlie": "delta"})"#;
    //     assert_command_stdout_contains!(command, &target);
    // }

}