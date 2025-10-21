use std::str::FromStr;

pub(crate) use chumsky::Parser;
use chumsky::{prelude::*, text::newline};

use crate::{
    prelude::Keyboard,
    window::msg::{Message, WindowMessage},
};

// #[derive(Debug)]
// pub(crate) struct Selector<'a> {
//     pub(crate) selector: Option<&'a str>,
//     pub(crate) value: &'a str,
// }

#[derive(Debug)]
pub(crate) enum Selector<'a> {
    Class(&'a str),
    Caption(&'a str),
}

#[derive(Debug)]
pub(crate) struct Command<'a> {
    pub(crate) selector: Option<Selector<'a>>,
    pub(crate) messages: Vec<Message>,
}

pub(crate) fn parser<'src>()
-> impl Parser<'src, &'src str, Vec<Command<'src>>, extra::Err<Rich<'src, char>>> {
    // ident is a atom
    let ident = text::ident().labelled("identifier");

    let count = text::int(10)
        .map(|s: &str| s.parse::<u32>().unwrap_or(1))
        .or_not();

    let selector = ident
        .padded()
        .then_ignore(just(":"))
        .or_not()
        .padded()
        .then(text::ident())
        .delimited_by(just("\""), just("\""))
        .map(|(selector, value): (Option<&str>, _)| match selector {
            Some(s) if s.starts_with("cl") => Selector::Class(value),
            Some(s) if s.starts_with("cap") => Selector::Caption(value),
            _ => Selector::Class(value),
        })
        .or_not();

    let command = ident
        .then_ignore(just(' ').or_not())
        .then(count)
        .map(|(s, count): (&str, Option<u32>)| Message {
            msg: WindowMessage::KeyDown(
                Keyboard::from_str(&s.to_uppercase())
                    .map(|k| u32::from(k))
                    .unwrap_or(0),
            ),
            count: count.unwrap_or(1),
        })
        .delimited_by(just('{').or_not(), just('}').or_not());

    let command_seq = command.repeated().collect();

    selector
        .padded()
        .then(command_seq)
        .then(newline().ignored().or(just(';').ignored()))
        .map(|((sel, cmds), _)| Command {
            selector: sel,
            messages: cmds,
        })
        .padded()
        .repeated()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use ariadne::{Color, Label, Report, ReportKind, Source};

    #[test]
    fn test_parser() {
        let src = r#"
        "caption:HELP" {UP 3}{DOWN}{down}{down};
        "caption:HELP" {UP 4}{DOWN}{down}{down}; "class:HELP" {UP 5}{DOWN}{down}{down}
        "class: HELP" {UP 6}{DOWN}{down}{down};
        "HELP" {UP 6}{DOWN}{down}{down}
        {UP 6}{DOWN}{down}{down}
        "#;

        let result = dbg!(parser().parse(&src));
        let errors = result.errors();

        for e in errors {
            Report::build(ReportKind::Error, ((), e.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new(((), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(Source::from(&src))
                .unwrap()
        }
    }

    #[test]
    fn test_parser_oneline() {
        let src = r#""caption:HELP" {UP 3}{DOWN}{down}{down}"#;

        let result = dbg!(parser().parse(&src));
        let errors = result.errors();

        for e in errors {
            Report::build(ReportKind::Error, ((), e.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new(((), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(Source::from(&src))
                .unwrap()
        }
    }
}
