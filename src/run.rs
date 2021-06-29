//! Run the app

use std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;

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
    let mut templater = crate::templating::kinds::tera::default_with_args(&args);

    // Add templates
    crate::templating::kinds::tera::add_template_files_via_args(&mut templater, &args)
    .chain_err(|| "add_template_files_via_args")?;

    // Add default template as needed
    if !crate::templating::kinds::tera::has_template(&templater) {
        crate::templating::kinds::tera::add_template_default(&mut templater)
        .chain_err(|| "add_template_default")?;
    }

    // Process each page
    if let Some(path_buf_list) = &args.input_path_buf_list {
        for path_buf in path_buf_list {
            do_path(
                &args,
                &templater,
                &path_buf
            )?;
        }
    };
    Ok(())
}

fn do_path(args: &Args, templater: &Templater, input_file_path: &PathBuf) -> Result<()> {
    trace!("do path(…) → input_file_path: {:?}", input_file_path);

    // Vet input file path
    vet_input_file_path_exists(&args, input_file_path)?;
    vet_input_file_path_metadata(&args, input_file_path)?;
    vet_input_file_path_extension(&args, input_file_path)?;
    debug!("input_file_path: {:?}", &input_file_path);

    // Calculate output file path
    let output_file_path = create_output_file_path(&args, &input_file_path)?;
    debug!("output_file_path: {:?}", &output_file_path);

    // Read content as Markdown text
    let content_as_markdown_text = read_content_as_markdown_text(&input_file_path)?;
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
    ::std::fs::write(&output_file_path, output_as_html_text)
    .chain_err(|| "write output")?;
    debug!("write file ok");

    info!("do path → success → input_file_path: {:?} output_file_path: {:?}", input_file_path, output_file_path);
    Ok(())
}

/// Vet input path exists.
///
/// Example:
///
/// ```
/// let input_file_path: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_exists(&input_file_path);
/// ```
///
fn vet_input_file_path_exists(_args: &Args, input_file_path: &PathBuf) -> Result<()> {
    if !input_file_path.exists() {
        bail!("input file path must exist. path: {:?}", input_file_path)
    }
    Ok(())
}

/// Vet input path metadata is file.
///
/// Example:
///
/// ```
/// let input_file_path: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_metadata(&input_file_path);
/// ```
///
fn vet_input_file_path_metadata(_args: &Args, input_file_path: &PathBuf) -> Result<()> {
    let metadata = ::std::fs::metadata(input_file_path)
    .chain_err(|| format!("input file path must have metadata. path: {:?}", input_file_path))?;
    if !metadata.is_file() {
        bail!("input file path must be a file. path: {:?}", input_file_path);
    }
    Ok(())
}

/// Vet input path name ends with "md" meaning Markdown format.
///
/// Example:
///
/// ```
/// let input_file_path: PathBuf = PathBuf::from("example.md");
/// vet_input_file_path_extension(&input_file_path);
/// ```
///
fn vet_input_file_path_extension(args: &Args, input_file_path: &PathBuf) -> Result<()> {
    if let Some(a) = &args.input_extension {
        if let Some(b) = &input_file_path.extension() {
            if a != &String::from(b.to_string_lossy()) {
                bail!("input file path extension must be \"{:?}\" but is \"{:?}. path: {:?}", a, b, input_file_path);
            }
        }
    }
    Ok(())
}

/// Create output path, either via args or changing input path extension from "md" to "html".
///
/// Example:
///
/// ```
/// let args = Args::default();
/// let input_file_path: PathBuf = PathBuf::from("example.md");
/// let output_file_path: PathBuf = create_output_file_path(&args, &input_file_path).unwrap();
/// assert_eq!(output_file_path, "example.html");
/// ```
///
fn create_output_file_path(args: &Args, input_file_path: &PathBuf) -> Result<PathBuf> {
    if let Some(path) = &args.output_file_path {
        return Ok(path.to_path_buf())
    }
    let mut path = PathBuf::from(input_file_path);
    path.set_extension(
        match &args.output_extension {
            Some(extension) => extension,
            _ => "html",
        }
    );
    Ok(path)
}

/// Read content as Markdown text.
///
/// Example:
///
/// ```
/// let input_file_path: PathBuf = PathBuf::from("example.md");
/// let content_as_markdown: String = read_content_as_markdown(&input_file_path);
/// ```
///
fn read_content_as_markdown_text(input_file_path: &PathBuf) -> Result<String> {
    ::std::fs::read_to_string(input_file_path)
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
    fn test_vet_input_file_path_exists_x_ok() {
        let args = Args::default();
        let input_file_path = TESTS_DIR.join("function").join("vet_input_file_path_exists").join("example.txt");
        let x = vet_input_file_path_exists(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_exists_x_err() {
        let args = Args::default();
        let input_file_path = TESTS_DIR.join("function").join("vet_input_file_path_exists").join("missing");
        let x = vet_input_file_path_exists(&args, &input_file_path);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_file_path_metadata_x_ok() {
        let args = Args::default();
        let input_file_path = TESTS_DIR.join("function").join("vet_input_file_path_metadata").join("example.txt");
        let x = vet_input_file_path_metadata(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_metadata_x_err() {
        let args = Args::default();
        let input_file_path = TESTS_DIR.join("function").join("vet_input_file_path_metadata").join("missing");
        let x = vet_input_file_path_metadata(&args, &input_file_path);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_file_path_extension_x_default_x_ok() {
        let args = Args::default();
        let input_file_path = PathBuf::from("example.md");
        let x = vet_input_file_path_extension(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_extension_x_default_x_err() {
        let args = Args::default();
        let input_file_path = PathBuf::from("example.invalid");
        let x = vet_input_file_path_extension(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_extension_x_custom_x_ok() {
        let mut args = Args::default();
        args.input_extension = Some(String::from("alpha"));
        let input_file_path = PathBuf::from("example.alpha");
        let x = vet_input_file_path_extension(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_extension_x_custom_x_err() {
        let mut args = Args::default();
        args.input_extension = Some(String::from("alpha"));
        let input_file_path = PathBuf::from("example.bravo");
        let x = vet_input_file_path_extension(&args, &input_file_path);
        assert!(x.is_err());
    }

    #[test]
    fn test_create_output_file_path_x_default() {
        let input_file_path = PathBuf::from("example.md");
        let args = Args::default();
        let x = create_output_file_path(&args, &input_file_path);
        assert_eq!(x.unwrap().to_string_lossy(), "example.html");
    }

    #[test]
    fn test_create_output_file_path_x_output_extension() {
        let mut args = Args::default();
        args.output_extension = Some(String::from("alpha"));
        let input_file_path = PathBuf::from("example.md");
        let x = create_output_file_path(&args, &input_file_path);
        assert_eq!(x.unwrap().to_string_lossy(), "example.alpha");
    }

    #[test]
    fn test_create_output_file_path_x_output_file_path() {
        let mut args = Args::default();
        args.output_file_path = Some(PathBuf::from("alpha"));
        let input_file_path = PathBuf::from("example.md");
        let x = create_output_file_path(&args, &input_file_path);
        assert_eq!(x.unwrap().to_string_lossy(), "alpha");
    }


    #[test]
    fn test_read_content_as_markdown_text() {
        let input_file_path: PathBuf = TESTS_DIR.join("function").join("read_content_as_markdown_text").join("example.md");
        let content_as_markdown: String = read_content_as_markdown_text(&input_file_path).unwrap();
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
