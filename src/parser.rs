use nom::{alpha, alphanumeric, rest};
use std::vec::Vec;

use template::Chunk;

named!(var_name<Vec<u8>>,
    map!(
        recognize!(
            do_parse!(
                alpha >>
                many0!(alt!(alphanumeric | tag!("_"))) >>
                ()
            )
        ), Vec::from
    )
);

named!(var<Chunk>,
    map!(
        ws!(
            delimited!(
                tag!("{%"),
                var_name,
                tag!("%}")
            )
        ),
        |v| Chunk::Var(Vec::from(v))
    )
);

named!(literal<Chunk>,
    map!(
        alt_complete!(take_until!("{%") | rest),
        |s| Chunk::Str(Vec::from(s))
    )
);

named!(pub template<Vec<Chunk>>,
    many0!(
        alt!(var | literal)
    )
);


#[cfg(test)]
mod test {
    use super::*;
    use template::Chunk;

    macro_rules! str {
        ($s: expr) => (Chunk::Str($s.to_vec()))
    }

    macro_rules! var {
        ($s: expr) => (Chunk::Var($s.to_vec()))
    }

    #[test]
    fn basic_parsing() {
        let (_, parsed) = template(b"Hello, {% who %}!").unwrap();
        let chunks = vec![str!(b"Hello, "), var!(b"who"), str!(b"!")];
        assert_eq!(chunks, parsed);
    }

    #[test]
    fn escaped_parsing() {
        let (_, parsed) = template(b"Hello, {%% who %}!").unwrap();
        let chunks = vec![str!(b"Hello, "), str!(b"{%"), str!(b" who %}!")];
        assert_eq!(chunks, parsed)
    }
}
