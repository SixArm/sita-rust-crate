use std::path::PathBuf;
use walkdir::WalkDir;
use crate::app::args::Args;
use crate::state::state_trait::StateTrait;
use crate::templater::templater_trait::TemplaterTrait;
use crate::f::from_markdown_str_into_html_string::*;
use crate::f::from_html_str_into_headline_str::*;
use crate::cook_file::cook_file;

pub (crate) fn cook_dir<T: TemplaterTrait> (
    args: &Args, 
    templater: Option<&T>, 
    input: &PathBuf, 
    output: &PathBuf
) -> Result<(), Error> {
    trace!("Cook directory.");
    trace!("cook_dir ➡ args: {:?}, templater: {:?}, input: {:?}, output: {:?}", args, templater, input, output);
    vet_input(input)?;
    vet_output(output)?;
    let output_file_name_extension = match &args.output_file_name_extension {
        Some(x) => x,
        None => &crate::app::args::OUTPUT_FILE_NAME_EXTENSION_AS_PATH_BUF,
    };
    for dir_entry in WalkDir::new(&input) {
        match dir_entry {
            Ok(dir_entry) => {
                let file_type = dir_entry.file_type();
                debug!("cook_dir ➡ dir_entry: {:?}, file_type: {:?}", dir_entry, file_type);
                if file_type.is_file() {
                    match dir_entry.path().strip_prefix(&input) {
                        Ok(path) => {
                            let input_entry = input.join(path);
                            let mut output_entry = output.join(path); 
                            output_entry.set_extension(output_file_name_extension);
                            cook_file(
                                &args,
                                templater,
                                &input_entry,
                                &output_entry,
                            )
                            .map_or_else(
                                |err| Err(Error::CookFile(err)),
                                |()| Ok(())
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
                    trace!("cook_dir ➡ input: {:?}, output: {:?}, dir entry is a dir", input, output);
                } else {
                    //TODO handle the corner cases
                    trace!("cook_dir ➡ input: {:?}, output: {:?}, skip because dir entry is not a dir nor a file", input, output);
                }
            },
            Err(err) => {
                return Err(Error::WalkDir(err))
            }
        }
    }
    Ok(())
}

fn vet_input(
    input: &PathBuf
) -> Result<(), Error>  {
    if !input.exists() { return Err(Error::InputMustExist { input: input.to_owned() }) }
    if !input.is_dir() { return Err(Error::InputMustBeDir { input: input.to_owned() }) }
    Ok(())    
}

fn vet_output(
    output: &PathBuf
) -> Result<(), Error>  {
    if !output.exists() { return Err(Error::OutputMustExist { output: output.to_owned() }) }
    if !output.is_dir() { return Err(Error::OutputMustBeDir { output: output.to_owned() }) }
    Ok(())    
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("InputMustExist ➡ input: {input:?}")]
    InputMustExist {
        input: PathBuf
    },

    #[error("InputMustBeDir ➡ input: {input:?}")]
    InputMustBeDir {
        input: PathBuf
    },

    #[error("OutputMustExist ➡ output: {output:?}")]
    OutputMustExist {
        output: PathBuf
    },

    #[error("OutputMustBeDir ➡ output: {output:?}")]
    OutputMustBeDir {
        output: PathBuf
    },

    #[error("CookFile ➡ {0:?}")]
    CookFile(crate::cook_file::Error),

    #[error("WalkDir ➡ {0:?}")]
    WalkDir(walkdir::Error),

    #[error("StripPrefixError ➡ input_dir: {input_dir:?}, dir_entry: {dir_entry:?}, strip_prefix_error: {strip_prefix_error:?}")]
    StripPrefixError {
        input_dir: PathBuf,
        dir_entry: walkdir::DirEntry,
        strip_prefix_error: std::path::StripPrefixError,  
    },

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
        .join("cook_dir")
    );

    #[test]
    fn test() {
        let args = Args::default();
        let templater: Option<&TemplaterWithHandlebars<'_>> = None;
        let input = DIR.join("input");
        let input_files = [
            input.join("alfa.md"),
            input.join("bravo.md"),
        ];
        input_files.into_iter().for_each(|file| assert!(file.exists(), "file: {:?}", file));
        let output = DIR.join("output");
        let output_files = [
            output.join("alfa.html"),
            output.join("bravo.html"),
        ];
        output_files.into_iter().for_each(|file| assert_ok!(remove_file_if_exists(file)));
        let result = cook_dir(&args, templater, &input, &output);
        assert_ok!(result);
        //TODO
    }

}

