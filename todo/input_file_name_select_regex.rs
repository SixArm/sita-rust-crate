/// Vet input file path buffer name matches the select regex, if any..
///
/// Example:
///
/// ```
/// let regex_string = "^e.*md$";
/// let input: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_buf_select_regex(&input);
/// ```
///
fn vet_input_file_path_buf_select_regex(args: &Args, input: &PathBuf) -> Result<()> {
    // TODO: optimize and uplift
    if let Some(s) = &args.input_file_name_select_regex_string {
        let regex = Regex::new(&s)
        .map_or_else(
            |err| InputFileNameSelectRegex::Regex(err),
            |val: regex::Captures| val.ok()
        )?;
        if regex.is_match(&input.to_string_lossy()) {
            return Ok(());
        } else {
            bail!("vet_input_file_path_buf_select_regex input: {:?} regex: {:?}", input, regex);
        }
    }
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum InputFileNameSelectRegexError {
    #[error ("regex")]
    Regex(regex::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    #[test]
    fn test_vet_input_file_path_buf_select_regex_x_default_x_ok() {
        let args = Args::default();
        let input_file_path_buf = PathBuf::from("example.md");
        let x = vet_input_file_path_buf_select_regex(&args, &input_file_path_buf);
        x.unwrap();
    }

    #[test]
    fn test_vet_input_file_path_buf_select_regex_x_default_x_err() {
        let args = Args::default();
        let input_file_path_buf = PathBuf::from("example.invalid");
        let x = vet_input_file_path_buf_select_regex(&args, &input_file_path_buf);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_file_path_buf_select_regex_x_custom_x_ok() {
        let mut args = Args::default();
        args.input_file_name_select_regex_string = Some(String::from("^e.*a$"));
        let input_file_path_buf = PathBuf::from("example.alfa");
        let x = vet_input_file_path_buf_select_regex(&args, &input_file_path_buf);
        x.unwrap();
    }

    #[test]
    fn test_vet_input_file_path_buf_select_regex_x_custom_x_err() {
        let mut args = Args::default();
        args.input_file_name_select_regex_string = Some(String::from("^e.*a$"));
        let input_file_path_buf = PathBuf::from("example.bravo");
        let x = vet_input_file_path_buf_select_regex(&args, &input_file_path_buf);
        assert!(x.is_err());
    }

}