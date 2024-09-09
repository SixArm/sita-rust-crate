//! Run the app

use std::path::PathBuf;
use crate::app::args::Args;
use crate::state::state_trait::StateTrait;
use crate::templater::templater_trait::TemplaterTrait;
//use crate::f::from_path_buf_into_sibling_extension::*;
use crate::f::from_markdown_str_into_html_string::*;
use crate::f::from_html_str_into_headline_str::*;

pub (crate) fn cook_file<T: TemplaterTrait> (
    args: &Args,
    templater: &T,
    input: &PathBuf,
    output: &PathBuf
) -> Result<(), Error> {
    trace!("Cook file.");
    debug!("args: {:?}, templater: {:?}, input: {:?}, output: {:?}", args, templater, input, output);
    vet_input(input)?;
    let mix_text = read_input_into_mix_text(input)?;
    let (content_text, state) = parse_mix_to_content_text_and_state(mix_text)?;
    let input_html_text = convert_from_markdown_str_into_html_string(&content_text);
    let state = insert_state_variables(state, &input_html_text);
    let template_name = get_template_name(templater)?;    
    let output_html_text = render(templater, &template_name, &state)?;
    let output_html_text = crate::rewriting::lol::rewrite(&output_html_text);
    write_output(output, &output_html_text)?;
    Ok(())
}

fn vet_input(
    input: &PathBuf
) -> Result<(), Error>  {
    trace!("Vet input.");
    debug!("input: {:?}", input);
    if !input.exists() { return Err(Error::InputMustExist { input: input.to_owned() }) }
    if !input.is_file() { return Err(Error::InputMustBeFile { input: input.to_owned() }) }
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
    trace!("Set the state variables as needed.");
    debug!("state: {:?}, content: {:?}", state, html);
    if !state.contains_key("content") {
        trace!("Set the state variables: set `content` to the HTML String.");
        state.insert(String::from("content"), String::from(html));
    }
    if !state.contains_key("title") {
        trace!("Set the state variables: set `title` to the first headline.");
        let title = match from_html_str_into_headline_str(html) {
            Some(s) => s,
            None => "",
        };
        state.insert(String::from("title"), String::from(title));
    }
    state
}

fn get_template_name<T: TemplaterTrait>(
    templater: &T,
) -> Result<String, Error> {
    //TODO make dynamic; currently this implementation merely returns the first available name
    trace!("Get template name.");
    debug!("templater: {:?}", templater);
    let template_names = templater.template_names_as_set_str();
    match template_names.iter().next() {
        Some(&s) => Ok(String::from(s)),
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
    use crate::templater::templater_with_handlebars::TemplaterWithHandlebars;

    pub static DIR: Lazy<PathBuf> = Lazy::new(||
        crate::testing::TESTS_DIR
        .join("src")
        .join("cook_file")
    );

    #[test]
    fn test() {
        let args = Args::default();
        let mut templater = TemplaterWithHandlebars::new();
        // let result = templater.register_template_via_default();
        // assert_result_ok!(result);
        templater.register_template_via_default().expect("register_template_via_default");
        let input = DIR.join("example.md");
        let output = DIR.join("example.html");
        let expect = DIR.join("example.html=expect.html");
        let result = cook_file(&args, &templater, &input, &output);
        assert_result_ok!(result);
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