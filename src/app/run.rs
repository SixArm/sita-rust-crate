//! Run the app

use std::path::PathBuf;
//use std::ffi::OsStr;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;
use crate::f::from_path_buf_into_sibling::*;
use crate::state::state_trait::StateTrait;
use crate::state::state_with_btms::StateWithBTMS;
use crate::templater::templater_trait::TemplaterTrait;
use crate::templater::templater_with_handlebars::TemplaterWithHandlebars;
//use crate::templater::templater_with_liquid::TemplaterWithLiquid;
//use crate::templater::templater_with_tera::TemplaterWithTera;
use crate::f::from_html_str_into_headline_str::*;
use crate::f::from_pathable_string_into_list_path_buf::*;

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
    let mut templater = TemplaterWithHandlebars::new_with_args(&args);

    trace!("Add templates.");
    if let Some(template_list_pathable_string) = &args.template_list_pathable_string {
        for template_pathable_string in template_list_pathable_string {
            trace!("Add templates: template_list_pathable_string: {}", &template_pathable_string);
            for template_path_buf 
            in from_pathable_string_into_list_path_buf(&template_pathable_string)
            .chain_err(|| "from_pathable_string_into_list_path_buf template_pathable_string")?
            .iter()
            .filter(|&x| 
                x.is_file()
            ){
                trace!("Add templates: template_path_buf: {:?}", &template_path_buf);
                let name_as_os_str = template_path_buf.file_name()
                .chain_err(|| "template_path_buf.file_name cannot convert to OsStr")?;
                let name = name_as_os_str.to_string_lossy();
                templater.register_template_via_name_and_content_file(&name, &template_path_buf)
                .chain_err(|| "register_template_via_name_and_content_file")?;
            }
        }
    }

    trace!("Add default template as needed.");
    if !templater.contains_any_template() {
        templater.register_template_via_default()
        .chain_err(|| "register_template_via_default")?;
    }

    trace!("Add helpers.");
    if let Some(helper_list_pathable_string) = &args.helper_list_pathable_string {
        for helper_pathable_string in helper_list_pathable_string {
            trace!("Add helpers: helper_list_pathable_string: {}", &helper_pathable_string);
            for helper_path_buf 
            in from_pathable_string_into_list_path_buf(&helper_pathable_string)
            .chain_err(|| "from_pathable_string_into_list_path_buf helper_pathable_string")?
            .iter()
            .filter(|&x| 
                x.is_file()
            ){
                trace!("Add helpers: helper_path_buf: {:?}", &helper_path_buf);
                let name_as_os_str = helper_path_buf.file_name()
                .chain_err(|| "helper_path_buf.file_name cannot convert to OsStr")?;
                let name = name_as_os_str.to_string_lossy();
                templater.register_helper_via_name_and_content_file(&name, &helper_path_buf)
                .chain_err(|| "add_helper_via_name_and_content_file")?;
            }
        }
    }
    
    trace!("Prepare items in order to speed up processing.");
    let output_file_name_extension = match &args.output_file_name_extension {
        Some(x) => x,
        None => crate::app::args::OUTPUT_FILE_NAME_EXTENSION_AS_STR,
    };

    trace!("Process inputs.");
    if let Some(input_list_pathable_string) = &args.input_list_pathable_string {
        for input_pathable_string in input_list_pathable_string {
            trace!("Process inputs: input_pathable_string={}", input_pathable_string);
            for input_path_buf 
            in from_pathable_string_into_list_path_buf(&input_pathable_string)
            .chain_err(|| "from_pathable_string_into_list_path_buf input_pathable_string")?
            .iter()
            .filter(|&x| 
                x.is_file()
            ){
                trace!("Process inputs: input_path_buf: {:?}", input_path_buf);
                let output_path_buf = from_path_buf_into_sibling(&input_path_buf, &output_file_name_extension);
                trace!("Process inputs: output_path_buf: {:?}", output_path_buf);
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

fn do_path<T: TemplaterTrait>(
    _args: &Args, 
    templater: &T, 
    input: &PathBuf, 
    output: &PathBuf
) -> Result<()> {
    trace!("do path(…) → input: {:?}", input);

    trace!("Read content as mix text.");
    let content_as_mix_text = read_content_as_mix_text(&input)?;
    debug!("content_as_mix_text: {:?}", content_as_mix_text);

    trace!("Parse matter that holds variables.");
    // TODO refactor this section to use let(…), when it is stable.
    let content_as_markdown_text: String;
    let mut state_trait: Box<dyn StateTrait>;
    if let Ok(parsed) = crate::matter::matter_parser_mutex::parse_mix_text_to_content_text_and_state(&content_as_mix_text) {
        content_as_markdown_text = parsed.0;
        state_trait = parsed.1;
    } else {
        content_as_markdown_text = content_as_mix_text.into();
        state_trait = Box::new(StateWithBTMS::new());
    }
    debug!("state_trait: {:?}", &state_trait);

    trace!("Convert from Markdown text to HTML text");
    let content_as_html_text = convert_from_markdown_text_to_html_text(&content_as_markdown_text);
    debug!("content_as_html_text: {:?}", &content_as_html_text);

    trace!("Set the content HTML for the content template tag.");
    state_trait.insert(String::from("content"), content_as_html_text.clone());
    debug!("state_trait: {:?}", &state_trait);

    trace!("Set the state with special keys.");
    if !state_trait.contains_key("title") {
        if let Some(title) = from_html_str_into_headline_str(&content_as_html_text) {
            state_trait.contains_key_or_insert(String::from("title"), String::from(title));
        }
    }
    debug!("state_trait: {:?}" , &state_trait);

    trace!("Convert the state to a final state enum.");
    let state_enum = state_trait.to_state_enum();
    debug!("state_enum: {:?}" , &state_enum);

    trace!("Select the template name."); //TODO make dynamic
    let template_name = *templater.template_names_as_set_str().iter().next().expect("template_name");
    debug!("template_name: {:?}", &template_name);
 
    trace!("Render the template.");
    let output_as_html_text = templater.render_template_with_state_enum(&template_name, &state_enum)
    .chain_err(|| "render_template_with_state")?;
    debug!("output_as_html_text: {:?}", &output_as_html_text);

    trace!("Rewrite the HTML.");
    let output_as_html_text = crate::rewriting::lol::rewrite(&output_as_html_text);
    debug!("output_as_html_text: {:?}", &output_as_html_text);

    trace!("Write output file.");
    ::std::fs::write(&output, output_as_html_text)
    .chain_err(|| "write output")?;
    debug!("write file ok");

    info!("do path → success → input: {:?} output: {:?}", input, output);
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

    #[test]
    fn test_run() {
        //TODO
    }

    #[test]
    fn test_do_path() {
        //TODO
    }

    #[test]
    fn test_read_content_as_mix_text() {
        let input_file_path_buf = crate::test::TESTS_DIR.join("function").join("read_content_as_mix_text").join("example.md");
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
