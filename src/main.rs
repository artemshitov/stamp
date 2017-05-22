#[macro_use]
extern crate clap;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quick_error;
extern crate walkdir;

use clap::{App, AppSettings};

use std::env;
use std::path::{Path, PathBuf};

mod error;
mod file;
mod parser;
mod questions;
mod stamp;
mod template;
mod template_file;

use error::{Error, Result};
use template_file::RenderedFile;

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let source_arg = matches
        .value_of("source")
        .expect("Source is a required argument");
    let dest_arg = matches.value_of("destination").unwrap_or("");

    let source = find_source(source_arg)?;
    let dest = find_destination(dest_arg)?;

    let files = file::read_all_files(&source)?;
    let (templates, vars) = stamp::compile_templates(&files);

    let conf = questions::get_vars(&vars)?;
    let rendered = templates
        .iter()
        .map(|t| t.render(&conf))
        .collect::<Result<Vec<RenderedFile>>>()?;

    stamp::write_files(&dest, &rendered)?;

    Ok(())
}

pub fn find_source(source_arg: &str) -> Result<PathBuf> {
    let mut path = Path::new(source_arg).to_owned();
    if path.is_relative() {
        let in_current_dir = env::current_dir()?.join(&path);
        if in_current_dir.exists() {
            return Ok(in_current_dir);
        } else {
            path = env::home_dir()
                .ok_or(Error::HomeDirNotAccessible)?
                .join(".stamps")
                .join(&path);
        }
    }
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::StampNotFound)
    }
}

fn find_destination(dest_arg: &str) -> Result<PathBuf> {
    let path = Path::new(dest_arg).to_owned();
    if path.is_relative() {
        Ok(env::current_dir()?.join(&path))
    } else {
        Ok(path)
    }
}

fn main() {
    run().unwrap()
}
