//! Run the app

use std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;
use crate::fun::path_buf_to_sibling::*;

type Templater = ::tera::Tera;

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

    // Initialize configuration
    let _config: Config = ::confy::load("sita")
    .chain_err(|| "error: confy load")?;

    // Initialize arguments
    let args: Args = crate::app::clap::args();
    if args.test { println!("{:?}", args); }

    // Initialize templating
    let mut templater_handlebars = crate::templating::kinds::handlebars::default_with_args(&args);
    let mut templater = crate::templating::kinds::tera::default_with_args(&args);

    // Add templates
    crate::templating::kinds::tera::add_template_files_via_args(&mut templater, &args)
    .chain_err(|| "add_template_files_via_args")?;

    // Add default template as needed
    if !crate::templating::kinds::tera::has_template(&templater) {
        crate::templating::kinds::tera::add_template_default(&mut templater)
        .chain_err(|| "add_template_default")?;
    }

    // Prepare items in order to speed up processing
    let output_file_name_extension = match &args.output_file_name_extension {
        Some(x) => x,
        None => "html",
    };

    // Process each page
    if let Some(inputs) = &args.input_list_path_buf {
        for input in inputs {
            // Calculate output path
            // TODO Shift the queue of output path list
            let output = path_buf_to_sibling(&input, &output_file_name_extension);
            debug!("output: {:?}", &output);
            do_path(
                &args,
                &templater,
                &input,
                &output,
            )?;
        }
    };
    Ok(())
}

fn do_path(
    args: &Args, templater: &Templater, 
    input: &PathBuf, 
    output: &PathBuf
) -> Result<()> {
    trace!("do path(…) → input: {:?}", input);

    // Vet input file path buf
    vet_input_file_path_buf_exists(&args, input)?;
    vet_input_file_path_buf_metadata(&args, input)?;
    vet_input_file_path_buf_extension(&args, input)?;
    debug!("input: {:?}", &input);

    // Vet output file path buf
    //TODO implement
    debug!("output: {:?}", &output);

    // Read content as Markdown text
    let content_as_markdown_text = read_content_as_markdown_text(&input)?;
    debug!("content_as_markdown_text: {:?}", content_as_markdown_text);

    // Parse front matter that holds variables
    let (content_as_markdown_text, matter) = crate::markdown::matter::util::extract(&content_as_markdown_text);
    debug!("matter: {:?}", &matter);

    // Convert from Markdown text to HTML text
    let content_as_html_text = convert_from_markdown_text_to_html_text(&content_as_markdown_text);
    debug!("content_as_html_text: {:?}", &content_as_html_text);

    // Create templater context that holds variables
    let mut context = match matter {
        crate::markdown::matter::state::State::HTML(x) => {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "create context")?
        }
        crate::markdown::matter::state::State::JSON(x) =>  {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "create context")?
        }
        crate::markdown::matter::state::State::TOML(x) => {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "create context")?
        }            
        crate::markdown::matter::state::State::YAML(x) => {
            ::tera::Context::from_serialize(&x)
            .chain_err(|| "create context")?
        }, 
        crate::markdown::matter::state::State::None => {
            ::tera::Context::new()
        }
    };
    debug!("context: {:?}", &context);

    // Set the magic "content" key for the corresponding template tag "{{ content }}"
    context.insert("content", &content_as_html_text);
    debug!("context with content: {:?}", &context);

    // Select relevant template name
    let template_name = select_template_name(&args, &templater);
    debug!("template_name: {:?}", &template_name);

    // Render template with context
    let output_as_html_text = templater.render(&template_name, &context)
    .chain_err(|| "render template")?;
    debug!("output_as_html_text: {:?}", &output_as_html_text);

    // Write output
    debug!("write file");
    ::std::fs::write(&output, output_as_html_text)
    .chain_err(|| "write output")?;
    debug!("write file ok");

    info!("do path → success → input: {:?} output: {:?}", input, output);
    Ok(())
}

/// Vet input path buffer exists.
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

/// Read content as Markdown text.
///
/// Example:
///
/// ```
/// let input_file_path_buf: PathBuf = PathBuf::from("example.md");
/// let content_as_markdown: String = read_content_as_markdown(&input_file_path_buf);
/// ```
///
fn read_content_as_markdown_text(input_file_path_buf: &PathBuf) -> Result<String> {
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

/// Select the revelant template name.
///
/// Example:
///
/// ```
/// let templater = crate::templating::kinds::tera::default();
/// let template_name = select_template_name(&args, &templater);
/// assert_eq!(template_name, "default");
/// ```
///
fn select_template_name(args: &Args, templater: &Templater) -> String {
    trace!("template_name(…)");
    if let Some(s) = &args.template_name {
        s.clone()
    } else {
        crate::templating::kinds::tera::best_template_name(&templater)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;
    use ::lazy_static::lazy_static;
    use crate::app::args::Args;

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
    fn test_read_content_as_markdown_text() {
        let input_file_path_buf: PathBuf = TESTS_DIR.join("function").join("read_content_as_markdown_text").join("example.md");
        let content_as_markdown: String = read_content_as_markdown_text(&input_file_path_buf).unwrap();
        assert_eq!(content_as_markdown, "# alpha\nbravo");
    }

    #[test]
    fn test_convert_from_markdown_text_to_html_text() {
        let markdown_text: &str = "# alpha\nbravo\n";
        let html_text = convert_from_markdown_text_to_html_text(markdown_text);
        assert_eq!(html_text, "<h1>alpha</h1>\n<p>bravo</p>\n");
    }

    #[test]
    fn test_select_template_name_x_default() {
        let args = Args::default();
        let templater = crate::templating::kinds::tera::default();
        let template_name = select_template_name(&args, &templater);
        assert_eq!(template_name, "default");
    }

    #[test]
    fn test_select_template_name_x_custom() {
        // TODO
    }

}
