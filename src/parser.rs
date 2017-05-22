use nom::{IResult, alpha, alphanumeric};

use template::Chunk;

fn rest(i: &[u8]) -> IResult<&[u8], &[u8]> {
    IResult::Done(&[], i)
}

named!(var_name<&[u8], &[u8]>,
    recognize!(
        do_parse!(
            alpha >>
            many0!(alt!(alphanumeric | tag!("_"))) >>
            ()
        )
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
        Chunk::Var
    )
);

named!(literal<&[u8], Chunk>,
    map!(
        alt!(take_until!("{%") | rest),
        Chunk::Str
    )
);

named!(pub template<&[u8], Vec<Chunk>>,
    many0!(
        alt!(var | literal)
    )
);

