use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::string::String;

use error::Result;
use template::Template;

#[derive(Debug)]
pub struct TemplateFile {
    pub path: Template,
    pub body: Template,
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
