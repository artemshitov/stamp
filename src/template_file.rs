use std::collections::{HashSet, HashMap};
use std::path::{Path, PathBuf};
use std::string::String;

use error::Result;
use template::Template;

#[derive(Debug)]
pub struct TemplateFile<'a> {
    path: Template<'a>,
    body: Template<'a>,
}

#[derive(Debug)]
pub struct RenderedFile {
    pub path: PathBuf,
    pub body: Vec<u8>,
}

const PATH_MUST_BE_UTF: &'static str = "Path must be a valid Unicode value";

impl<'a> TemplateFile<'a> {
    pub fn parse(path: &'a Path, body: &'a [u8]) -> TemplateFile<'a> {
        let path_template = Template::parse(path.to_str().expect(PATH_MUST_BE_UTF).as_bytes());
        let body_template = Template::parse(body);

        TemplateFile {
            path: path_template,
            body: body_template,
        }
    }

    pub fn extract_vars(&self, target: &mut HashSet<&'a [u8]>) {
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

pub fn compile_templates(files: &[(PathBuf, Vec<u8>)]) -> (Vec<TemplateFile>, HashSet<&[u8]>) {
    let mut vars = HashSet::new();
    let mut templates = Vec::new();

    for &(ref path, ref content) in files {
        let file = TemplateFile::parse(path, content);
        file.extract_vars(&mut vars);
        templates.push(file);
    }

    (templates, vars)
}
