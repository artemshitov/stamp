use std::str;
use std::collections::{HashSet, HashMap};

use nom::IResult;

use error::Error;
use parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chunk<'a> {
    Str(&'a [u8]),
    Var(&'a [u8]),
}

#[derive(Debug)]
pub struct Template<'a> {
    chunks: Vec<Chunk<'a>>,
}

impl<'a> Template<'a> {
    pub fn parse(i: &'a [u8]) -> Template<'a> {
        match parser::template(i) {
            IResult::Done(_, o) => Template { chunks: o },
            _ => unreachable!(),
        }
    }

    pub fn render(&self, vars: &HashMap<&[u8], String>) -> Result<Vec<u8>, Error> {
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
