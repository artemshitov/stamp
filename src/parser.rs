use nom::{IResult, alpha, alphanumeric};
use std::vec::Vec;

use template::Chunk;

fn rest(i: &[u8]) -> IResult<&[u8], &[u8]> {
    IResult::Done(&[], i)
}

named!(var_name<&[u8], Vec<u8>>,
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

named!(var<&[u8], Chunk>,
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

named!(literal<&[u8], Chunk>,
    map!(
        alt_complete!(take_until!("{%") | rest),
        |s| Chunk::Str(Vec::from(s))
    )
);

named!(pub template<&[u8], Vec<Chunk>>,
    many0!(
        alt!(var | literal)
    )
);
