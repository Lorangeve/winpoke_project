use chumsky::prelude::*;

fn parser<'src>() -> impl Parser<'src, &'src str, ()> {
    text::ident().ignore_then(end())
}

fn main() {
    dbg!(parser().parse("Hello"));
}
