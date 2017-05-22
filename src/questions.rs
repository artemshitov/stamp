use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};
use std::string::String;

use error::Error;

fn ask(question: &str) -> Result<String, Error> {
    print!("{}: ", question);
    let _ = stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}

pub fn get_vars<'a>(questions: &'a HashSet<&'a [u8]>) -> Result<HashMap<&'a [u8], String>, Error> {
    let mut result = HashMap::new();
    for question in questions {
        let answer = ask(&String::from_utf8(question.to_vec())?)?;
        let _ = result.insert(*question, answer);
    }
    Ok(result)
}
