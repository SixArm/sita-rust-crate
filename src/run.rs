//! Run the app

use ::tera::Tera;
use ::std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;
use crate::templating::vars::Vars;


pub(crate) fn run() -> Result<()> {
    let _config: Config = ::confy::load("sita")
    .chain_err(|| "configuration load error")?;
    let args: Args = crate::app::clap::args();
    let tera: Tera = crate::templating::tera::init(&args)
    .chain_err(|| "init tera")?;
    let template_name = match &args.template_name {
        Some(s) => &*s,
        _ => crate::templating::tera::template_default_name(),
    };
    if let Some(paths) = &args.paths {
        for input_file_path in paths {
            do_path(
                &args, 
                &tera, 
                &template_name, 
                &input_file_path
            )?; 
        }
    };
    Ok(())
}

fn do_path(args: &Args, tera: &Tera, template: &str, input_file_path: &PathBuf) -> Result<()> {
    if args.verbose > 0 {
        info!("do path → start → input:{:?}", input_file_path);
    }
    vet_input_file_path_exists(&args, input_file_path)?;
    vet_input_file_path_metadata(&args, input_file_path)?;
    vet_input_extension(&args, input_file_path)?;
    let output_file_path = create_output_file_path(&args, &input_file_path)?;

    // Translate Markdown to HTML
    let input_as_markdown = ::std::fs::read_to_string(&input_file_path)
    .chain_err(|| format!("input path must be readable; path: {:?}", input_file_path))?;
    let content_as_html = markdown_to_html(&input_as_markdown);

    // Create variables
    let vars = Vars {
        title: Some("my title".into()),
        content: Some(content_as_html),
    };

    // Render Tera template that has {{ content }} slot for HTML string
    let context = ::tera::Context::from_serialize(&vars)
    .chain_err(|| "create context")?;
    let output_as_html = tera.render(template, &context)
    .chain_err(|| "render template")?;
    ::std::fs::write(&output_file_path, output_as_html)
    .chain_err(|| "write output")?;
    if args.verbose > 0 {
        info!("do path → success → input:{:?} output:{:?}", input_file_path, output_file_path);
    }
    Ok(())
}

/// Vet input path exists
fn vet_input_file_path_exists(_args: &Args, input_file_path: &PathBuf) -> Result<()> {
    if !input_file_path.exists() {
        bail!("input path must exist. path: {:?}", input_file_path)
    }
    Ok(())
}

/// Vet input path metadata is file
fn vet_input_file_path_metadata(_args: &Args, input_file_path: &PathBuf) -> Result<()> {
    let metadata = ::std::fs::metadata(input_file_path)
    .chain_err(|| format!("input path must have metadata. path: {:?}", input_file_path))?;
    if !metadata.is_file() {
        bail!("input path must be a file. path: {:?}", input_file_path);
    }
    Ok(())
}

/// Vet input path name ends with "md" meaning Markdown format
fn vet_input_extension(args: &Args, input_file_path: &PathBuf) -> Result<()> {
    if let Some(a) = &args.input_extension {
        if let Some(b) = &input_file_path.extension() {
            if a != &String::from(b.to_string_lossy()) {
                bail!("input extension must be \"{:?}\" but is \"{:?}. path: {:?}", a, b, input_file_path);
            }
        }
    }
    Ok(())
}

/// Create output path, either via args or changing input path extension from "md" to "html"
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

/// Translate Markdown text to HTML text
fn markdown_to_html(input_as_markdown: &str) -> String {
    let parser = crate::markdown::markdown_parser::parser(&*input_as_markdown);
    let mut content_as_html = String::new();
    pulldown_cmark::html::push_html(&mut content_as_html, parser);
    content_as_html
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::app::args::Args;

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
        let input_file_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "files", "input.md"].iter().collect();
        let args = Args::default();
        let x = vet_input_file_path_exists(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_exists_x_err() {
        let input_file_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "files", "missing"].iter().collect();
        let args = Args::default();
        let x = vet_input_file_path_exists(&args, &input_file_path);
        assert!(x.is_err());
    }
        
    #[test]
    fn test_vet_input_file_path_metadata_x_ok() {
        let input_file_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "files", "input.md"].iter().collect();
        let args = Args::default();
        let x = vet_input_file_path_metadata(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_file_path_metadata_x_err() {
        let input_file_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "files", "missing"].iter().collect();
        let args = Args::default();
        let x = vet_input_file_path_metadata(&args, &input_file_path);
        assert!(x.is_err());
    }

    #[test]
    fn test_vet_input_extension_x_ok() {
        let input_file_path = PathBuf::from("example.md");
        let args = Args::default();
        let x = vet_input_extension(&args, &input_file_path);
        assert!(x.is_ok());
    }

    #[test]
    fn test_vet_input_extension_x_err() {
        let input_file_path = PathBuf::from("example.md");
        let mut args = Args::default();
        args.input_extension = Some(String::from("alpha"));
        let x = vet_input_extension(&args, &input_file_path);
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
        let input_file_path = PathBuf::from("example.md");
        let mut args = Args::default();
        args.output_extension = Some(String::from("alpha"));
        let x = create_output_file_path(&args, &input_file_path);
        assert_eq!(x.unwrap().to_string_lossy(), "example.alpha");
    }

    #[test]
    fn test_create_output_file_path_x_output_file_path() {
        let input_file_path = PathBuf::from("example.md");
        let mut args = Args::default();
        args.output_file_path = Some(PathBuf::from("alpha"));
        let x = create_output_file_path(&args, &input_file_path);
        assert_eq!(x.unwrap().to_string_lossy(), "alpha");
    }

}
