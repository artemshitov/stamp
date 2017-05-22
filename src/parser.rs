use nom::IResult;

use template::Chunk;

fn rest(i: &[u8]) -> IResult<&[u8], &[u8]> {
    IResult::Done(&[], i)
}

named!(var<&[u8], Chunk>,
    map!(
        delimited!(
            tag!("{%"),
            take_until!("%}"),
            tag!("%}")
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
