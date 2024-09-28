//! Command line argument parsing (CLAP) for the application.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approaches:
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
use crate::types::{list::*, map::*};
use std::path::PathBuf;

/// Create a clap app.
pub fn app() -> Command {
    trace!("app");
    Command::new("Sita")
    .name(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::new("input")
        .help("An input path.\nExample file: --input \"example.html\" …\nExample directory: --input \"examples/\" …\nExample glob: --input \"examples/**/*\" …")
        .short('i')
        .long("input")
        .alias("inputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .value_parser(clap::value_parser!(PathBuf))
        .action(clap::ArgAction::Append)
        .num_args(1..)
    )
    .arg(Arg::new("output")
        .help("An output path.\nExample file: --output \"example.html\" …\nExample directory: --output \"examples/\" …\nExample glob: --output \"examples/**/*\" …")
        .short('o')
        .long("output")
        .alias("outputs")
        .value_name("FILE | DIRECTORY | GLOB")
        .value_parser(clap::value_parser!(PathBuf))
        .action(clap::ArgAction::Append)
        .num_args(1..)
    )
    .arg(Arg::new("template")
        .help("A template path.\nExample file: --template \"example.html\" …\nExample directory: --template \"examples/\" …\nExample glob: --template \"examples/**/*\" …")
        .short('t')
        .long("template")
        .alias("templates")
        .value_name("FILE | DIRECTORY | GLOB")
        .value_parser(clap::value_parser!(PathBuf))
        .action(clap::ArgAction::Append)
        .num_args(1..)
    )
    .arg(Arg::new("extra")
        .help("An extra path such as for helpers, scripts, utilities, etc.\nExample file: --extra \"example.rhai\" …\nExample directory: --extra \"extras/\" …\nExample glob: --extra \"extras/**/*\" …")
        .short('e')
        .long("extra")
        .alias("extras")
        .value_name("FILE | DIRECTORY | GLOB")
        .value_parser(clap::value_parser!(PathBuf))
        .action(clap::ArgAction::Append)
    )
    .arg(Arg::new("set")
        .help("Set a variable name to a value.\nExample: --set pi 3.1415 …")
        .short('s')
        .long("set")
        .num_args(2)
        .value_names(&["NAME", "VALUE"])
        .value_parser(clap::value_parser!(String))
        .action(clap::ArgAction::Append)
    )
    .arg(Arg::new("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .long("test")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(Arg::new("output_file_name_extension")
        .help("The output file name extension.\nDefault: \"html\".\nExample: --output-extension \"html\"")
        .long("output-extension")
        .value_name("EXTENSION")
        .value_parser(clap::value_parser!(PathBuf))
        .action(clap::ArgAction::Append)
    )
    .arg(Arg::new("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::Count)
    )
}

/// Create an Args struct initiated with the clap App settings.
pub fn args() -> Args {
    trace!("args");
    let matches = app().get_matches();
    trace!("args ➡ matches: {:?}", matches);

    let input_list: Option<List<PathBuf>> = match matches.get_many("input") {
        Some(paths) => Some(paths.map(|path: &PathBuf| path.to_owned()).collect()),
        None => None,
    };

    let output_list: Option<List<PathBuf>> = match matches.get_many("output") {
        Some(paths) => Some(paths.map(|path: &PathBuf| path.to_owned()).collect()),
        _ => None,
    };

    let output_file_name_extension: Option<PathBuf> = match matches.get_one::<PathBuf>("output_file_name_extension") {
        Some(x) => Some(x.to_owned()),
        _ => None,
    };

    let extra_list: Option<List<PathBuf>> = match matches.get_many("extra") {
        Some(paths) => Some(paths.map(|path: &PathBuf| path.to_owned()).collect()),
        _ => None,
    };

    let settings = match matches.get_occurrences("set") {
        Some(occurrences) => {
            // TODO: refactor & optimize
            let occurrences: Vec<Vec<&String>> = occurrences.map(Iterator::collect).collect();
            let map: Map<String, String> =
            occurrences.into_iter().map(|occurrence|
                (
                    occurrence[0].to_owned(),
                    occurrence[1].to_owned()
                )
            ).collect();
            Some(map)
        },
        None => None,
    };

    let template_list: Option<List<PathBuf>> = match matches.get_many("template") {
        Some(paths) => Some(paths.map(|path: &PathBuf| path.to_owned()).collect()),
        _ => None,
    };

    let test = matches.get_flag("test");

    let log_level = match matches.get_count("verbose") {
        0 => None,
        1 => Some(::log::Level::Error),
        2 => Some(::log::Level::Warn),
        3 => Some(::log::Level::Info),
        4 => Some(::log::Level::Debug),
        5 => Some(::log::Level::Trace),
        _ => Some(::log::Level::Trace),
    };

    let args = Args {
        input_list: input_list,
        log_level: log_level,
        output_list: output_list,
        output_file_name_extension: output_file_name_extension,
        settings: settings,
        template_list: template_list,
        extra_list: extra_list,
        test: test,
    };

    trace!("args ➡ args: {:?}", args);
    args
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::testing::*;
    use assertables::*;

    // Test that the special argument `--test` is working.
    //
    // This test must succeed in order for any of the rest of the tests here to
    // succeed, because the `--test` argument turns on the runtime output to stdout,
    // which includes a typical print debug of the entire `args` structure.
    //
    #[test]
    fn test_test() {
        let mut command = std::process::Command::new(&*COMMAND_OS);
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
    #[test]
    fn test_verbose() {
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test"], r#" log_level: None"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "-v"], r#" log_level: Some(Error)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "-vv"], r#" log_level: Some(Warn)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "-vvv"], r#" log_level: Some(Info)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "-vvvv"], r#" log_level: Some(Debug)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "-vvvvv"], r#" log_level: Some(Trace)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "--verbose"], r#" log_level: Some(Error)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose"], r#" log_level: Some(Warn)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Info)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Debug)"#);
        assert_program_args_stdout_contains!(&*COMMAND_OS, &["--test", "--verbose", "--verbose", "--verbose", "--verbose", "--verbose"], r#" log_level: Some(Trace)"#);
    }

    // #[path = "util.rs"]
    // mod util;
    // use util::*;

    #[test]
    fn test_input() {

        //// Test with one key and one value

        let s = "alfa";

        let target = format!(" input_list: Some([\"{}\"])", &s);

        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-i", &s]);
        assert_command_stdout_contains!(command, &target);
        
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--input", &s]);
        assert_command_stdout_contains!(command, &target);

        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--inputs", &s]);
        assert_command_stdout_contains!(command, &target);

        //// Test with multiple keys and multiple values
        
        let s1 = "alfa";
        let s2 = "bravo";
        let s3 = "charlie";
        let s4 = "delta";

        let target = format!(" input_list: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-i", &s1, &s2, "--input", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--input", &s1, &s2, "--input", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--inputs", &s1, &s2, "--inputs", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

    }

    #[test]
    fn test_output() {

        //// Test with one key and one value
        
        let s = "alfa";

        let target = format!(" output_list: Some([\"{}\"])", &s);

        // Test short `-o`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-o", &s]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--output`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--output", &s]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--outputs`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--outputs", &s]);
        assert_command_stdout_contains!(command, &target);

        //// Test with multiple keys and multiple values
        
        let s1 = "alfa";
        let s2 = "bravo";
        let s3 = "charlie";
        let s4 = "delta";

        let target = format!(" output_list: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

        // Test short `-o`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-o", &s1, &s2, "-o", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--output`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--output", &s1, &s2, "--output", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--outputs`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--outputs", &s1, &s2, "--outputs", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

    }

    #[test]
    fn test_clap_output_file_name_extension() {
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--output-extension", "alfa"]);
        let target = r#" output_file_name_extension: Some("alfa")"#;
        assert_command_stdout_contains!(command, &target);
    }

    #[test]
    fn test_template() {

        //// Test with one key and one value

        let s = "alfa";

        let target = format!(" template_list: Some([\"{}\"", &s);

        // Test short `-t`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-t", &s]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--template`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--template", &s]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--templates`
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--templates", &s]);
        assert_command_stdout_contains!(command, &target);

        //// Test with multiple keys and multiple values

        let s1 = "alfa";
        let s2 = "bravo";
        let s3 = "charlie";
        let s4 = "delta";

        let target = format!(" template_list: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

        // Test short `-t` with multiple keys and multiple values
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "-t", &s1, &s2, "-t", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test long `--template` with multiple keys and multiple values
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--template", &s1, &s2, "--template", &s3, &s4]);
        assert_command_stdout_contains!(command, &target);

        // Test alias `--templates` with multiple keys and multiple values
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--templates", &s1, &s2, "--templates", &s3, &s4]);
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

    #[test]
    fn test_set() {
        let mut command = std::process::Command::new(&*COMMAND_OS);
        command.args(&["--test", "--set", "alfa", "bravo", "--set", "charlie", "delta"]);
        let target = r#" settings: Some({"alfa": "bravo", "charlie": "delta"})"#;
        assert_command_stdout_contains!(command, &target);
    }

}