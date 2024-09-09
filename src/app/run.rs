//! Run the app

use std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::types::list::*;
use crate::templater::templater_trait::TemplaterTrait;
use crate::templater::templater_with_handlebars::TemplaterWithHandlebars;
//use crate::templater::templater_with_liquid::TemplaterWithLiquid;
//use crate::templater::templater_with_tera::TemplaterWithTera;
//use crate::f::from_path_buf_into_sibling_extension::*;
use walkdir::WalkDir;

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
pub(crate) fn run() -> Result<(), Error> {
    trace!("run");
    let _config = initialize_configuration()?;
    let args = initialize_arguments();
    let templater = initialize_templater(&args)?;
    cook_all(&args, &templater)
}

fn initialize_configuration() -> Result<Config, Error> {
    trace!("initialize_configuration");
    match confy::load("sita", None) {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::Confy(err)),
    }
}

fn initialize_arguments() -> Args {
    trace!("initialize_arguments");
    let args: Args = crate::app::clap::args();
    if args.test { 
        println!("{:?}", args);
        println!("log level: {:?}", args.log_level); 
    }
    args
}

fn initialize_templater(args: &Args) -> Result<TemplaterWithHandlebars, Error> {
    trace!("initialize_templater");
    let mut templater = TemplaterWithHandlebars::new_with_args(&args);
    initialize_templater_templates(&args, &mut templater)?;
    initialize_templater_default(&args, &mut templater)?;
    initialize_templater_helpers(&args, &mut templater)?;
    Ok(templater)
}

fn initialize_templater_templates(
    args: &Args,
    templater: &mut TemplaterWithHandlebars
) -> Result<(), Error> {
    trace!("initialize_templater_templates");

    if let Some(template_list) = &args.template_list {
        for template_path_buf in template_list.iter().filter(|&x| x.is_file()) {
            trace!("initialize_templater_templates ➡ template_path_buf: {:?}", template_path_buf);
            //TODO optimize
            let name: String = match template_path_buf.file_name() {
                Some(x) => x.to_string_lossy().into(),
                None => crate::app::args::FILE_NAME_IS_NONE_AS_STR.into(),
            };
            let content_text = std::fs::read_to_string(&template_path_buf)
            .map_or_else(
                |err| Err(
                    Error::InitializeTemplaterTemplate {
                        name: name.to_owned(),
                        template_path_buf: template_path_buf.to_owned(),
                        debug: format!("{:?}", err),
                    }
                ),
                |x| Ok(x),
            )?;
            templater.register_template_via_name_and_content_text(&name, &content_text)
            .map_or_else(
                |err| Err(
                    Error::InitializeTemplaterTemplate {
                        name: name.to_owned(),
                        template_path_buf: template_path_buf.to_owned(),
                        debug: format!("{:?}", err),
                    }
                ),
                |()| Ok(())
            )?
        }
    }
    Ok(())
}

fn initialize_templater_default(
    _args: &Args,
    templater: &mut TemplaterWithHandlebars
) -> Result<(), Error> {
    trace!("initialize_templater_default");
    if !templater.contains_any_template() {
        templater.register_template_via_default()
        .map_or_else(
            //TODO fix the error so it returns an object, not a string.
            // Currently the code forces a string to work around the issue "borrowed data escapes outside of function"
            |err| Err(Error::RegisterTemplateViaDefault { debug: format!("{:?}", err) }),
            |()| Ok(())
        )?
    }
    Ok(())
}

fn initialize_templater_helpers(
    args: &Args,
    templater: &mut TemplaterWithHandlebars,
) -> Result<(), Error> {
    trace!("initialize_templater_helpers");
    if let Some(extra_list) = &args.extra_list {
        for extra_path_buf in extra_list.iter().filter(|&x| x.is_file()) {
            trace!("initialize_templater_helpers ➡ extra_path_buf: {:?}", extra_path_buf);
            //TODO borrow
            let name: String = match extra_path_buf.file_name() {
                Some(x) => x.to_string_lossy().into(),
                None => crate::app::args::FILE_NAME_IS_NONE_AS_STR.into(),
            };
            templater.handlebars.register_script_helper_file(&name, extra_path_buf)
                .map_or_else(
                    |err| Err(Error::InitializeTemplaterExtra { name: name, extra_path_buf: extra_path_buf.to_owned(), debug: format!("{:?}", err) }),
                    |()| Ok(())
                )?
        }
    }
    Ok(())
}

fn cook_all(args: &Args, templater: &TemplaterWithHandlebars) -> Result<(), Error> {
    trace!("cook_all ➡ args.input_list: {:?}, args.output_list: {:?}", &args.input_list, &args.output_list);
    if let (
        Some(input_list),
        Some(output_list)
    ) = (
        &args.input_list,
        &args.output_list
    ){
        trace!("cook_all ➡ input_list len: {}, output_list len: {}", input_list.len(), output_list.len());
        vet_input_output_list_length(input_list, output_list)?;
        for i in 0..input_list.len() {
            let input = &input_list[i];
            let output = &output_list[i];
            cook_one(args, templater, input, output)?
        }
    } else {
        trace!("cook_all ➡ missing input/output lists");
    }
    Ok(())
}

fn vet_input_output_list_length(input_list: &List<PathBuf>, output_list: &List<PathBuf>) -> Result<(), Error> {
    trace!("vet_input_output_list_length ➡ input: {:?}, output: {:?}", input_list, output_list);
    if input_list.len() == output_list.len() {
        Ok(())
    } else {
        Err(Error::InputOutputListLength {
            input_list: input_list.to_owned(),
            output_list: output_list.to_owned(),
        })
    }
}

fn cook_one(args: &Args, templater: &TemplaterWithHandlebars, input: &PathBuf, output: &PathBuf) -> Result<(), Error> {
    trace!("cook_one ➡ input: {:?}, output: {:?}", input, output);

    if input.is_dir() {
        if output.is_file() {
            return Err(Error::CookOneInputIsDirButOutputIsFile { 
                input: input.to_owned(), 
                output: output.to_owned() 
            })
        }
        if output.exists() {
            return Err(Error::CookOneInputIsDirButOutputExists { 
                input: input.to_owned(), 
                output: output.to_owned() 
            })
        }
        return cook_one_dir(
            &args,
            templater,
            input,
            output,
        )
    }
    
    if input.is_file() {
        if output.is_dir() {
            return Err(Error::CookOneInputIsFileButOutputIsDir { 
                input: input.to_owned(), 
                output: output.to_owned() 
            })
        }
        if output.exists() {
            return Err(Error::CookOneInputIsFileButOutputExists { 
                input: input.to_owned(), 
                output: output.to_owned() 
            })
        }
        return crate::cook_file::cook_file(
            &args,
            templater,
            input,
            output,
        )
        .map_or_else(
            |err| Err(Error::CookFile {
                input: input.to_owned(), 
                output: output.to_owned(),
                err: err,
            }),
            |()| Ok(())
        )
    }

    Err(Error::CookOneInputIsNotDirAndIsNotFile { 
        input: input.to_owned()
    })
    
}

fn cook_one_dir(args: &Args, templater: &TemplaterWithHandlebars, input: &PathBuf, output: &PathBuf) -> Result<(), Error> {
    trace!("cook_one_dir ➡ input: {:?}, output: {:?}", input, output);
    let output_file_name_extension = match &args.output_file_name_extension {
        Some(x) => x,
        None => &crate::app::args::OUTPUT_FILE_NAME_EXTENSION_AS_PATH_BUF,
    };
    for dir_entry in WalkDir::new(&input) {
        match dir_entry {
            Ok(dir_entry) => {
                if dir_entry.file_type().is_file() {
                    trace!("cook_one_dir ➡ input: {:?}, output: {:?}, dir entry is a file", input, output);
                    match dir_entry.path().strip_prefix(&input) {
                        Ok(path) => {
                            let input_entry = input.join(path);
                            let mut output_entry = output.join(path); 
                            output_entry.set_extension(output_file_name_extension);
                            cook_one(
                                &args,
                                templater,
                                &input_entry,
                                &output_entry,
                            )?
                        },
                        Err(error) => {
                            return Err(Error::StripPrefixError {
                                input_dir: input.to_owned(),
                                dir_entry: dir_entry.to_owned(),
                                strip_prefix_error: error.to_owned(),
                            });
                        }
                    }
                } else
                if dir_entry.file_type().is_dir() {
                    //TODO handle this e.g. make the corresponding directory
                    trace!("cook_one_dir ➡ input: {:?}, output: {:?}, dir entry is a dir", input, output);
                } else {
                    //TODO handle the corner cases
                    trace!("cook_one_dir ➡ input: {:?}, output: {:?}, skip because dir entry is not a dir nor a file", input, output);
                }
            },
            Err(err) => {
                return Err(Error::Walk { 
                    input: input.to_owned(), 
                    output: output.to_owned(), 
                    walkdir_error: err
                });
            }
        }
    }
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Confy ➡ {0:?}")]
    Confy(confy::ConfyError),

    #[error("RegisterTemplateViaDefault ➡ {debug:?}")]
    RegisterTemplateViaDefault {
        debug: String
    },

    #[error("InputOutputListLength ➡ input_list: {input_list:?}, output_list: {output_list:?}")]
    InputOutputListLength {
        input_list: List<PathBuf>,
        output_list: List<PathBuf>,
    },

    #[error("InitializeTemplaterTemplate ➡ name: {name:?}, template_path_buf: {template_path_buf:?}, debug: {debug:?}")]
    InitializeTemplaterTemplate {
        name: String,
        template_path_buf: PathBuf,
        debug: String,
    },

    #[error("InitializeTemplaterExtra ➡ name: {name:?}, extra_path_buf: {extra_path_buf:?}, debug: {debug:?}")]
    InitializeTemplaterExtra {
        name: String,
        extra_path_buf: PathBuf,
        debug: String,
    },

    #[error("Walk ➡ input: {input:?}, output: {output:?}, walkdir_error: {walkdir_error:?}")]
    Walk {
        input: PathBuf,
        output: PathBuf,
        walkdir_error: walkdir::Error,
    },

    #[error("StripPrefixError ➡ input_dir: {input_dir:?}, dir_entry: {dir_entry:?}, strip_prefix_error: {strip_prefix_error:?}")]
    StripPrefixError {
        input_dir: PathBuf,
        dir_entry: walkdir::DirEntry,
        strip_prefix_error: std::path::StripPrefixError,  
    },

    #[error("CookOne ➡ input {input:?}, output {output:?}")]
    CookOne {
        input: PathBuf,
        output: PathBuf
    },

    #[error("CookOneInputIsDirButOutputIsFile ➡ input {input:?}, output {output:?}")]
    CookOneInputIsDirButOutputIsFile {
        input: PathBuf,
        output: PathBuf
    },

    #[error("CookOneInputIsDirButOutputExists ➡ input {input:?}, output {output:?}")]
    CookOneInputIsDirButOutputExists {
        input: PathBuf,
        output: PathBuf
    },

    #[error("CookOneInputIsFileButOutputIsDir ➡ input {input:?}, output {output:?}")]
    CookOneInputIsFileButOutputIsDir {
        input: PathBuf,
        output: PathBuf
    },

    #[error("CookOneInputIsFileButOutputExists ➡ input {input:?}, output {output:?}")]
    CookOneInputIsFileButOutputExists {
        input: PathBuf,
        output: PathBuf
    },

    #[error("CookOneInputIsNotDirAndIsNotFile ➡ input {input:?}")]
    CookOneInputIsNotDirAndIsNotFile {
        input: PathBuf,
    },    

    #[error("CookFile ➡ input {input:?}, output {output:?}, err: {err:?}")]
    CookFile {
        input: PathBuf,
        output: PathBuf,
        err: crate::cook_file::Error,
    },

}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_run() {
        //TODO
    }

}

// cSpell:ignore walkdir
