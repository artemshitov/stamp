use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};

use error::Error;
use file::write_file;
use template_file::{RenderedFile, TemplateFile};

pub fn find_stamp(path: &Path) -> Result<PathBuf, Error> {
    let mut path = path.to_owned();
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

pub fn render_templates(templates: &[TemplateFile],
                        vars: &HashMap<&[u8], String>)
                        -> Result<Vec<RenderedFile>, Error> {
    let mut result = Vec::new();
    for template in templates {
        result.push(template.render(vars)?);
    }
    Ok(result)
}

pub fn write_files(root: &Path, rendered: &[RenderedFile]) -> Result<(), Error> {
    let root = root.to_owned();
    for f in rendered {
        write_file(&root.join(&f.path), &f.body)?;
    }
    Ok(())
}
