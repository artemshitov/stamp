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
