use std::collections::HashSet;
use std::path::{Path, PathBuf};

use error::Result;
use file::write_file;
use template_file::{RenderedFile, TemplateFile};

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

pub fn write_files(root: &Path, rendered: &[RenderedFile]) -> Result<()> {
    let root = root.to_owned();
    for f in rendered {
        write_file(&root.join(&f.path), &f.body)?;
    }
    Ok(())
}
