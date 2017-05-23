use std::collections::{HashSet, HashMap};

use error::{Error, Result};
use parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chunk<'a> {
    Str(&'a [u8]),
    Var(&'a [u8]),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Template<'a> {
    chunks: Vec<Chunk<'a>>,
}

impl<'a> Template<'a> {
    pub fn parse(i: &'a [u8]) -> Result<Template<'a>> {
        Ok(Template { chunks: parser::template(i).to_full_result()? })
    }

    pub fn render(&self, vars: &HashMap<&[u8], String>) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        for chunk in &self.chunks {
            match *chunk {
                Chunk::Str(s) => buf.extend_from_slice(s),
                Chunk::Var(v) => {
                    buf.extend_from_slice(vars.get(v).ok_or(Error::VarNotFound)?.as_bytes())
                }
            }
        }
        Ok(buf)
    }

    pub fn extract_vars(&self, target: &mut HashSet<&'a [u8]>) {
        for chunk in &self.chunks {
            if let Chunk::Var(v) = *chunk {
                target.insert(v);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_parsing() {
        let parsed = Template::parse(b"Hello, {% who %}!").unwrap();
        let chunks = vec![Chunk::Str(b"Hello, "), Chunk::Var(b"who"), Chunk::Str(b"!")];
        assert_eq!(Template { chunks: chunks }, parsed);
    }
}

