use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::string::String;

use error::Result;
use template::Template;

#[derive(Debug)]
pub struct TemplateFile {
    path: Template,
    body: Template,
}

#[derive(Debug)]
pub struct RenderedFile {
    pub path: PathBuf,
    pub body: Vec<u8>,
}

const PATH_MUST_BE_UTF: &'static str = "Path must be a valid Unicode value";

impl TemplateFile {
    pub fn parse(path: &Path, body: &[u8]) -> Result<TemplateFile> {
        let path_template = Template::parse(path.to_str().expect(PATH_MUST_BE_UTF).as_bytes())?;
        let body_template = Template::parse(body)?;

        Ok(TemplateFile {
               path: path_template,
               body: body_template,
           })
    }

    pub fn extract_vars<'a>(&'a self, target: &mut HashSet<&'a [u8]>) {
        self.path.extract_vars(target);
        self.body.extract_vars(target);
    }

    pub fn render(&self, vars: &HashMap<&[u8], String>) -> Result<RenderedFile> {
        let path = PathBuf::from(String::from_utf8(self.path.render(vars)?)?);
        let body = self.body.render(vars)?;

        Ok(RenderedFile {
               path: path,
               body: body,
           })
    }
}

pub fn compile_templates(files: &[(PathBuf, Vec<u8>)]) -> Result<Vec<TemplateFile>> {
    files
        .iter()
        .map(|&(ref path, ref content)| TemplateFile::parse(path, content))
        .collect::<Result<Vec<TemplateFile>>>()
}
