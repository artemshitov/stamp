use std::collections::HashMap;
use std::io::{Write, stdin, stdout};
use std::string::String;

use error::Result;
use template::Chunk;
use template_file::TemplateFile;

fn ask(question: &str) -> Result<String> {
    print!("{}: ", question);
    let _ = stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}

pub fn get_vars(templates: &[TemplateFile]) -> Result<HashMap<&[u8], String>> {
    let mut result = HashMap::new();
    for tf in templates {
        for t in &[&tf.path, &tf.body] {
            for chunk in &t.chunks {
                if let Chunk::Var(ref v) = *chunk {
                    if !result.contains_key(v.as_slice()) {
                        let answer = ask(&String::from_utf8(v.clone())?)?;
                        let _ = result.insert(v.as_slice(), answer);
                    }
                }
            }
        }
    }
    Ok(result)
}
