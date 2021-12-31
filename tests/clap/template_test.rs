use std::process::Command;

#[path = "util.rs"]
mod util;
use util::*;

const dir = "template_list_pathable_string";

const s1: &str = format!("{}/{}", &dir, "a/**/*");
const s2: &str = format!("{}/{}", &dir, "b/**/*");
const s3: &str = format!("{}/{}", &dir, "c/**/*");
const s4: &str = format!("{}/{}", &dir, "d/**/*");

const target: &str = format!(" template_list_pathable_string: Some([\"{}\", \"{}\", \"{}\", \"{}\"])", &s1, &s2, &s3, &s4);

#[test]
fn test_template_x_short() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "-t", &s1, &s2, "-t", &s3, &s4], 
        &target
    );
}

#[test]
fn test_template_x_long() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--template", &s1, &s2, "--template", &s3, &s4], 
        &target
    );
}

#[test]
fn test_template_x_alias() {
    assert_command_stdout_contains(
        COMMAND, 
        &["--test", "--templates", &s1, &s2, "--templates", &s3, &s4], 
        &target
    );
}

// #[test]
// fn test_clap_template_glob_to_template_path_set() {
//     let dir = "from_set_pathable_string_into_set_path_buf/";
//     assert_command_stdout_contains(
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
