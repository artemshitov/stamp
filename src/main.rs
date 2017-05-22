#[macro_use]
extern crate clap;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quick_error;
extern crate walkdir;

use std::env;
use std::path::{Path, PathBuf};

mod error;
mod file;
mod parser;
mod questions;
mod stamp;
mod template;
mod template_file;

use clap::{App, AppSettings};

use error::Error;

fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let source_arg = matches.value_of("source").ok_or(Error::SourceNotProvided)?;
    let dest_arg = matches.value_of("destination").unwrap_or("");

    let source = stamp::find_stamp(Path::new(source_arg))?;
    let dest = find_destination(dest_arg)?;

    let files = file::read_all_files(&source)?;
    let (templates, vars) = stamp::compile_templates(&files);

    let conf = questions::get_vars(&vars)?;
    let rendered = stamp::render_templates(&templates, &conf)?;

    stamp::write_files(&dest, &rendered)?;

    Ok(())
}

fn find_destination(dest_arg: &str) -> Result<PathBuf, Error> {
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
