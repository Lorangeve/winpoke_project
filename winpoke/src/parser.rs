use chumsky::{extra::Err, prelude::*};

use crate::window::msg::{Message, WindowMessage};

#[derive(Debug)]
struct Selector<'a> {
    selector: Option<&'a str>,
    value: &'a str,
}

#[derive(Debug)]
struct Command<'a> {
    selector: Option<Selector<'a>>,
    commands: Vec<Message>,
}

fn parser<'src>() -> impl Parser<'src, &'src str, Vec<Command<'src>>, extra::Err<Rich<'src, char>>>
{
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
        .map(|(selector, value)| Selector { selector, value })
        .or_not();

    let command = ident
        .then_ignore(just(' ').or_not())
        .then(count)
        .or(ident.map(|s: &str| (s, None)))
        .map(|(s, count)| match s.to_uppercase().as_str() {
            "UP" => Message {
                msg: WindowMessage::KeyDown(0x26),
                count: count.unwrap_or(1),
                ..Default::default()
            },
            "DOWN" => Message {
                msg: WindowMessage::KeyDown(0x28),
                count: count.unwrap_or(1),
                ..Default::default()
            },
            "LEFT" => Message {
                msg: WindowMessage::KeyDown(0x25),
                count: count.unwrap_or(1),
                ..Default::default()
            },
            "RIGHT" => Message {
                msg: WindowMessage::KeyDown(0x27),
                count: count.unwrap_or(1),
                ..Default::default()
            },
            other => Message {
                msg: WindowMessage::Char(other.chars().next().unwrap_or_default()),
                count: count.unwrap_or(1),
                ..Default::default()
            },
        })
        .delimited_by(just('{').or_not(), just('}').or_not());

    let command_seq = command.padded().repeated().collect();

    selector
        .padded()
        .then(command_seq)
        .map(|(sel, cmds)| Command {
            selector: sel,
            commands: cmds,
        })
        .then_ignore(just(";").padded())
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
            "caption:HELP" {UP 4}{DOWN}{down}{down};
            "class:HELP" {UP 5}{DOWN}{down}{down};
            "class: HELP" {UP 6}{DOWN}{down}{down};
            "HELP" {UP 6}{DOWN}{down}{down};
            {UP 6}{DOWN}{down}{down};
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
}
