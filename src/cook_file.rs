use std::path::PathBuf;
use crate::app::args::Args;
use crate::state::state_trait::StateTrait;
use crate::templater::templater_trait::TemplaterTrait;
use crate::f::from_markdown_str_into_html_string::*;
use crate::f::from_html_str_into_headline_str::*;
use crate::f::from_html_str_into_paragraph_str::*;

pub (crate) fn cook_file<T: TemplaterTrait> (
    args: &Args,
    templater: Option<&T>,
    input: &PathBuf,
    output: &PathBuf,
) -> Result<(), Error> {
    trace!("Cook file.");
    debug!("cook_file ➡ args: {:?}, templater: {:?}, input: {:?}, output: {:?}", args, templater, input, output);
    vet_input(input)?;
    vet_output(output)?;
    let mix_text = read_input_into_mix_text(input)?;
    let (content_text, state) = parse_mix_to_content_text_and_state(mix_text)?;
    let input_html_text = convert_from_markdown_str_into_html_string(&content_text);
    let output_html_text = match templater {
        Some(templater) => {
            let state = insert_state_variables(state, &input_html_text);
            let template_name = get_template_name(templater)?;    
            render(templater, &template_name, &state)?
        },
        None => {
            input_html_text
        }
    };
    let output_html_text = crate::rewriting::lol::rewrite(&output_html_text);
    write_output(output, &output_html_text)?;
    Ok(())
}

fn vet_input(
    input: &PathBuf
) -> Result<(), Error>  {
    if !input.exists() { return Err(Error::InputMustExist { input: input.to_owned() }) }
    if !input.is_file() { return Err(Error::InputMustBeFile { input: input.to_owned() }) }
    Ok(())    
}

fn vet_output(
    output: &PathBuf
) -> Result<(), Error>  {
    if output.exists() { return Err(Error::OutputMustNotExist { output: output.to_owned() }) }
    Ok(())    
}

fn read_input_into_mix_text(
    input: &PathBuf
) -> Result<String, Error> {
    trace!("Read input into mix text.");
    debug!("input: {:?}", input);
    std::fs::read_to_string(input)
    .map_or_else(
        |err| Err(Error::ReadMixText { input: input.to_owned(), err: err }),
        |s| Ok(String::from(s.trim_end())) //TODO optimize to &str
    )
}

fn parse_mix_to_content_text_and_state(
    mix_text: String
) -> Result<(String, Box<dyn StateTrait>), Error> {
    trace!("Parse mix text into markdown text and state.");
    debug!("mix_text: {:?}", mix_text);
    crate::matter::matter_parser_mutex::parse_mix_text_to_content_text_and_state(&mix_text)
    .map_or_else(
        |err| Err(Error::ParseMixText { mix_text: mix_text, err: err }),
        |x| Ok(x)
    )
}

fn convert_from_markdown_str_into_html_string(
    markdown_str: &str
) -> String {
    trace!("Convert from markdown text into HTML text.");
    debug!("markdown_str: {:?}", markdown_str);
    from_markdown_str_into_html_string(&markdown_str)
}

fn insert_state_variables(
    mut state: Box<dyn StateTrait>, 
    html: &str
) -> Box<dyn StateTrait> {
    trace!("Set state variables as needed.");
    state = insert_state_content(state, html);
    state = insert_state_title(state, html);
    state = insert_state_description(state, html);
    state
}

fn insert_state_content(
    mut state: Box<dyn StateTrait>, 
    html: &str
) -> Box<dyn StateTrait> {
    trace!("insert_state_content");
    if !state.contains_key("content") {
        state.insert(String::from("content"), String::from(html));
    }
    state
}

fn insert_state_title(
    mut state: Box<dyn StateTrait>, 
    html: &str
) -> Box<dyn StateTrait> {
    trace!("insert_state_title");
    if !state.contains_key("title") {
        if let Some(s) = from_html_str_into_headline_str(html) {
            state.insert(String::from("title"), String::from(s));
        }
    }
    state
}

fn insert_state_description(
    mut state: Box<dyn StateTrait>, 
    html: &str
) -> Box<dyn StateTrait> {
    trace!("insert_state_description");
    if !state.contains_key("description") {
        if let Some(s) = from_html_str_into_paragraph_str(html) {
            state.insert(String::from("description"), String::from(s));
        }
    }
    state
}

fn get_template_name<T: TemplaterTrait>(
    templater: &T,
) -> Result<&str, Error> {
    //TODO make dynamic; currently this implementation merely returns the first available name
    trace!("Get template name.");
    debug!("templater: {:?}", templater);
    let template_names = templater.template_names_as_set_str();
    match template_names.iter().next() {
        Some(&s) => Ok(s),
        None => Ok(templater.template_name_default())
    }
}

fn render<T: TemplaterTrait>(
    templater: &T, 
    template_name: &str, 
    state: &Box<dyn StateTrait>
) -> Result<String, Error> {
    trace!("Render via the templater trait and state enum.");
    debug!("templater: {:?}, template_name: {:?}, state: {:?}", templater, template_name, state);
    templater.render_template_with_state_enum(&template_name, &state.to_state_enum())
    .map_or_else(
        |err| Err(Error::Render { 
            //templater: Box::new(templater), //TODO fix
            template_name: template_name.to_owned(), 
            err: format!("{:#?}", err)
        }),
        |val| Ok(val)
    )
}

fn write_output(
    output: &PathBuf, 
    contents: &str
) -> Result<(), Error> {
    trace!("Write output.");
    debug!("output: {:?}, contents: {:?}", output, contents);
    std::fs::write(&output, contents)
    .map_or_else(
        |err| Err(Error::Write {
            output: output.to_owned(),
            contents: contents.into(),
            err: err
        }),
        |val| Ok(val)
    )
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("InputMustExist ➡ input {input:?}")]
    InputMustExist {
        input: PathBuf,
    },

    #[error("InputMustBeFile ➡ input {input:?}")]
    InputMustBeFile {
        input: PathBuf,
    },

    #[error("OutputMustNotExist ➡ output {output:?}")]
    OutputMustNotExist {
        output: PathBuf,
    },

    #[error("ReadMixText ➡ input {input:?}, err {err:?}")]
    ReadMixText {
        input: PathBuf,
        err: std::io::Error,
    },

    #[error("ParseMixText ➡ mix_text {mix_text:?}")]
    ParseMixText {
        mix_text: String,
        err: crate::matter::matter_parser_mutex::Error,
    },

    #[error("Render ➡ template_name: {template_name:?}")] //TODO "Render ➡ templater: {templater:?}, template_name: {template_name:?}, err: {err:?}"
    Render {
        //templater: Box<dyn TemplaterTrait>, //TODO fix
        template_name: String,
        err: String //TODO improve to Box<dyn std::error::Error>
    },

    #[error("Write ➡ output: {output:?}, err {err:?}")]
    Write {
        output: PathBuf,
        contents: String,
        err: std::io::Error,
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::*;
    use once_cell::sync::Lazy;
    use crate::f::remove_file_if_exists::*;
    use crate::templater::templater_with_handlebars::TemplaterWithHandlebars;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("cook_file")
    );

    #[test]
    fn test_sans_templater() {
        let dir = DIR.join("test_sans_templater");
        let args = Args::default();
        let option_templater: Option<&TemplaterWithHandlebars<'_>> = None;
        let input = dir.join("example.md");
        let output = dir.join("example.html");
        let expect = dir.join("example.html=expect.html");
        assert_ok!(remove_file_if_exists(&output));
        let result = cook_file(&args, option_templater, &input, &output);
        assert_ok!(result);
        assert_fs_read_to_string_eq!(&output, &expect);
    }

    #[test]
    fn test_with_templater() {
        let dir = DIR.join("test_with_templater");
        let args = Args::default();
        let mut templater: TemplaterWithHandlebars<'_> = TemplaterWithHandlebars::new();
        templater.register_template_via_default().expect("register_template_via_default");
        let input = dir.join("example.md");
        let output = dir.join("example.html");
        let expect = dir.join("example.html=expect.html");
        assert_ok!(remove_file_if_exists(&output));
        let result = cook_file(&args, Some(&templater), &input, &output);
        assert_ok!(result);
        assert_fs_read_to_string_eq!(&output, &expect);
    }

    #[test]
    fn test_read_input_as_mix_text() {
        let input_file_path_buf = crate::testing::TESTS_DIR
        .join("src")
        .join("f")
        .join("read_content_as_mix_text")
        .join("example.md");
        let mix_text: String = super::read_input_into_mix_text(&input_file_path_buf).unwrap();
        assert_eq!(
            mix_text,
            "# alfa\nbravo"
        );
    }

    #[test]
    fn test_convert_from_markdown_str_into_html_string() {
        let markdown_str: &str = "# alfa\nbravo\n";
        let html_string: String = super::convert_from_markdown_str_into_html_string(markdown_str);
        assert_eq!(
            html_string,
            String::from("<h1>alfa</h1>\n<p>bravo</p>\n")
        );
    }

}
