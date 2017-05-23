use std::collections::{HashSet, HashMap};
use std::fmt;
use std::str;

use error::{Error, Result};
use parser;

#[derive(Clone, PartialEq, Eq)]
pub enum Chunk {
    Str(Vec<u8>),
    Var(Vec<u8>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Template {
    chunks: Vec<Chunk>,
}

impl Template {
    pub fn parse(i: &[u8]) -> Result<Template> {
        Ok(Template { chunks: parser::template(i).to_full_result()? })
    }

    pub fn render(&self, vars: &HashMap<&[u8], String>) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        for chunk in &self.chunks {
            match *chunk {
                Chunk::Str(ref s) => buf.extend(s),
                Chunk::Var(ref v) => {
                    buf.extend(vars.get(&v[..]).ok_or(Error::VarNotFound)?.as_bytes())
                }
            }
        }
        Ok(buf)
    }

    pub fn extract_vars<'a>(&'a self, target: &mut HashSet<&'a [u8]>) {
        for chunk in &self.chunks {
            if let Chunk::Var(ref v) = *chunk {
                target.insert(v);
            }
        }
    }
}

impl<'a> fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_str = |body: &[u8]| str::from_utf8(body).unwrap_or("[binary]").to_owned();
        match *self {
            Chunk::Str(ref b) => write!(f, "Str({:?})", to_str(b)),
            Chunk::Var(ref b) => write!(f, "Var({:?})", to_str(b)),
        }
    }
}
