//! Run the app

use std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;
use crate::fun::from_path_buf_into_sibling::*;
use crate::state::state::State;
use crate::state::state_enum::StateEnum;
use crate::state::state_with_html::StateWithHTML;
use crate::templating::templater::Templater;
use crate::templating::templater_with_tera::TemplaterWithTera;
use crate::fun::from_html_str_into_headline_str::*;
use crate::fun::from_pathable_string_into_list_path_buf::*;

/// Run everything.
///
/// Steps:
///
///   * Initialize configuration.
///
///   * Initialize arguments.
///
///   * Initialize templating.
///
//    * Process each page.
///
/// Example:
///
/// ```
/// run();
/// //-> Initialize everything then process each page
/// ```
///
pub(crate) fn run() -> Result<()> {
    trace!("run()");

    trace!("Initialize configuration.");
    let _config: Config = ::confy::load("sita")
    .chain_err(|| "error: confy load")?;

    trace!("Initialize arguments.");
    let args: Args = crate::app::clap::args();
    if args.test { println!("{:?}", args); }

    trace!("Initialize templater.");
    let mut templater = TemplaterWithTera::new_with_args(&args);

    trace!("Add templates.");
    if let Some(template_list_pathable_string) = &args.template_list_pathable_string {
        for template_pathable_string in template_list_pathable_string {
            trace!("Add templates: template_list_pathable_string={}", &template_pathable_string);
            for template_path_buf in from_list_pathable_string_into_list_path_buf(&template_list_pathable_string) {
                trace!("Add templates: template_path_buf={:?}", &template_path_buf);
                let name_as_os_str = template_path_buf.file_name()
                .chain_err(|| "template_path_buf.file_name cannot convert to OsStr")?;
                let name = name_as_os_str.to_string_lossy();
                templater.add_template_via_name_and_file(&name, &template_path_buf)
                .chain_err(|| "add_template_via_name_and_file")?;
            }
        }
    }
    if !templater.has_template() {
        templater.add_template_via_default();
    }

    trace!("Prepare items in order to speed up processing.");
    let output_file_name_extension = match &args.output_file_name_extension {
        Some(x) => x,
        None => "html",
    };

    trace!("Process inputs.");
    if let Some(input_list_pathable_string) = &args.input_list_pathable_string {
        for input_pathable_string in input_list_pathable_string {
            trace!("Process inputs: input_pathable_string={}", input_pathable_string);
            for input_path_buf in from_pathable_string_into_list_path_buf(&input_pathable_string) {
                trace!("Process inputs: input_path_buf={:?}", input_path_buf);
                let output_path_buf = from_path_buf_into_sibling(&input_path_buf, &output_file_name_extension);
                trace!("Process inputs: output_path_buf={:?}", output_path_buf);
                do_path(
                    &args,
                    &templater,
                    &input_path_buf,
                    &output_path_buf,
                )?;
            }
        }
    }    
    
    Ok(())
}

fn do_path<T: Templater>(
    args: &Args, 
    templater: &T, 
    input: &PathBuf, 
    output: &PathBuf
) -> Result<()> {
    trace!("do path(…) → input: {:?}", input);

    trace!("Vet input file path buffer.");
    vet_input_file_path_buf_exists(&args, input)?;
    vet_input_file_path_buf_metadata(&args, input)?;
    vet_input_file_path_buf_extension(&args, input)?;
    debug!("input: {:?}", &input);

    trace!("Vet output file path buffer.");
    //TODO implement
    debug!("output: {:?}", &output);

    trace!("Read content as mix text.");
    let content_as_mix_text = read_content_as_mix_text(&input)?;
    debug!("content_as_mix_text={:?}", content_as_mix_text);

    trace!("Parse matter that holds variables.");
    // TODO refactor this section to use let(…), when it is stable.
    let content_as_markdown_text: String;
    let mut box_dyn_state: Box<dyn State>;
    if let Ok(parsed) = crate::matter::matter_parser_mutex::parse_mix_text_to_content_text_and_state(&content_as_mix_text) {
        content_as_markdown_text = parsed.0;
        box_dyn_state = parsed.1;
    } else {
        content_as_markdown_text = content_as_mix_text.into();
        box_dyn_state = Box::new(StateWithHTML::new());
    }
    debug!("box_dyn_state: {:?}", &box_dyn_state);

    trace!("Convert from Markdown text to HTML text");
    let content_as_html_text = convert_from_markdown_text_to_html_text(&content_as_markdown_text);
    debug!("content_as_html_text: {:?}", &content_as_html_text);

    trace!("Set the content HTML for the content template tag.");
    box_dyn_state.insert(String::from("content"), content_as_html_text.clone());

    trace!("Set the state with special keys.");
    if !box_dyn_state.contains_key("title") {
        if let Some(title) = from_html_str_into_headline_str(&content_as_html_text) {
            box_dyn_state.contains_key_or_insert(String::from("title"), String::from(title));
        }
    }
    debug!("box_dyn_state={:?}" , &box_dyn_state);

    trace!("Convert the state to a final state enum.");
    let state_enum = box_dyn_state.to_state_enum();

    trace!("Select the template name."); //TODO make dynamic
    let template_name = *templater.template_names_as_set_str().iter().next().expect("template_name");
    debug!("template_name: {:?}", &template_name);
 
    trace!("Render the template.");
    let output_as_html_text = templater.render_template_with_state_enum(&template_name, &state_enum)
    .chain_err(|| "render_template_with_state")?;

    trace!("Write output file.");
    debug!("write file …");
    ::std::fs::write(&output, output_as_html_text)
    .chain_err(|| "write output")?;
    debug!("write file ok");

    info!("do path → success → input: {:?} output: {:?}", input, output);
    Ok(())
}

/// Vet input file path buffer exists.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_buf_exists(&input);
/// ```
///
fn vet_input_file_path_buf_exists(_args: &Args, input: &PathBuf) -> Result<()> {
    if !input.exists() {
        bail!("input must exist. path: {:?}", input)
    }
    Ok(())
}

/// Vet input file path buffer metadata is file.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_buf_metadata(&input);
/// ```
///
fn vet_input_file_path_buf_metadata(_args: &Args, input: &PathBuf) -> Result<()> {
    let metadata = ::std::fs::metadata(input)
    .chain_err(|| format!("input must have metadata. path: {:?}", input))?;
    if !metadata.is_file() {
        bail!("input must be a file. path: {:?}", input);
    }
    Ok(())
}

/// Vet input file path buffer name ends with the correct extension,
/// typically "md" meaning Markdown format.
///
/// Example:
///
/// ```
/// let input: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_buf_extension(&input);
/// ```
///
fn vet_input_file_path_buf_extension(args: &Args, input: &PathBuf) -> Result<()> {
    if let Some(a) = &args.input_file_name_extension {
        if let Some(b) = &input.extension() {
            if a != &String::from(b.to_string_lossy()) {
                bail!("input extension must be \"{:?}\" but is \"{:?}\". path: {:?}", a, b, input);
            }
        }
    }
    Ok(())
}

/// Read content as mix text i.e. text that contains both Markdown and variables.
///
/// Example:
///
/// ```
/// let input_file_path_buf: PathBuf = PathBuf::from("example.md");
/// let content_as_markdown: String = read_content_as_markdown(&input_file_path_buf);
/// ```
///
fn read_content_as_mix_text(input_file_path_buf: &PathBuf) -> Result<String> {
    ::std::fs::read_to_string(input_file_path_buf)
    .map(|s| s.trim_end().to_string())
    .map_err(|e| Error::with_chain(e, "something went wrong"))
}

/// Convert from Markdown text to HTML text.
///
/// Example:
///
/// ```
/// let markdown_text: &str = "# alpha\nbravo\n";
/// let html_text = convert_from_markdown_text_to_html_text(markdown);
/// assert_eq!(html, "<h1>alpha</h1>\n<p>bravo</p>\n");
/// ```
///
fn convert_from_markdown_text_to_html_text(markdown_text: &str) -> String {
    let parser = crate::markdown::markdown_parser::parser(markdown_text);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    html_text
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;
    use ::lazy_static::lazy_static;
    use crate::app::args::Args;
    use crate::templating::templater_with_tera::TemplaterWithTera;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }

    #[test]
    fn test_run() {
        //TODO
    }

    #[test]
    fn test_do_path() {
        //TODO
    }

    #[test]
    fn test_vet_input_file_path_buf_exists_x_ok() {
        let args = Args::default();
        let input_file_path_buf = TESTS_DIR.join("function").join("vet_input_file_path_buf_exists").join("example.txt");
        let x = vet_input_file_path_buf_exists(&args, &input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_exists_x_err() {
        let args = Args::default();
        let input_file_path_buf = TESTS_DIR.join("function").join("vet_input_file_path_buf_exists").join("missing");
        let x = vet_input_file_path_buf_exists(&args, &input_file_path_buf);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_file_path_buf_metadata_x_ok() {
        let args = Args::default();
        let input_file_path_buf = TESTS_DIR.join("function").join("vet_input_file_path_buf_metadata").join("example.txt");
        let x = vet_input_file_path_buf_metadata(&args, &input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_metadata_x_err() {
        let args = Args::default();
        let input_file_path_buf = TESTS_DIR.join("function").join("vet_input_file_path_buf_metadata").join("missing");
        let x = vet_input_file_path_buf_metadata(&args, &input_file_path_buf);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_file_path_buf_extension_x_default_x_ok() {
        let args = Args::default();
        let input_file_path_buf = PathBuf::from("example.md");
        let x = vet_input_file_path_buf_extension(&args, &input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_extension_x_default_x_err() {
        let args = Args::default();
        let input_file_path_buf = PathBuf::from("example.invalid");
        let x = vet_input_file_path_buf_extension(&args, &input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_extension_x_custom_x_ok() {
        let mut args = Args::default();
        args.input_file_name_extension = Some(String::from("alpha"));
        let input_file_path_buf = PathBuf::from("example.alpha");
        let x = vet_input_file_path_buf_extension(&args, &input_file_path_buf);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_buf_extension_x_custom_x_err() {
        let mut args = Args::default();
        args.input_file_name_extension = Some(String::from("alpha"));
        let input_file_path_buf = PathBuf::from("example.bravo");
        let x = vet_input_file_path_buf_extension(&args, &input_file_path_buf);
        assert!(x.is_err());
    }

    #[test]
    fn test_read_content_as_mix_text() {
        let input_file_path_buf: PathBuf = TESTS_DIR.join("function").join("read_content_as_mix_text").join("example.md");
        let content_as_mix_text: String = read_content_as_mix_text(&input_file_path_buf).unwrap();
        assert_eq!(content_as_mix_text, "# alpha\nbravo");
    }

    #[test]
    fn test_convert_from_markdown_text_to_html_text() {
        let markdown_text: &str = "# alpha\nbravo\n";
        let html_text = convert_from_markdown_text_to_html_text(markdown_text);
        assert_eq!(html_text, "<h1>alpha</h1>\n<p>bravo</p>\n");
    }

}
