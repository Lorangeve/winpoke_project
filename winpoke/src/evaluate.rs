// use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::parser::{Parser, Selector, parser};
use crate::prelude::Result;
use crate::window::WindowInfo;

pub fn eval(s: &str) -> Result<()> {
    let commands = dbg!(parser().parse(&s));

    for command in commands.unwrap() {
        match &command.selector {
            Some(Selector::Class(value)) => {
                let windows = WindowInfo::all_windows()?
                    .into_iter()
                    .filter(|w| w.class_name == *value)
                    .collect::<Vec<WindowInfo>>();

                for window in windows {
                    // window.set_foreground_window()?;
                    dbg!(window).send_message_seq(&command.messages)?;
                }
                continue;
            }
            Some(Selector::Caption(value)) => {
                let windows: Vec<WindowInfo> = WindowInfo::all_windows()?
                    .into_iter()
                    .filter(|w| w.caption == *value)
                    .collect();

                for window in windows {
                    // window.set_foreground_window()?;
                    window.send_message_seq(&command.messages)?;
                }
                continue;
            }
            None => {
                continue;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let src = r#"
            "RegEdit_RegEdit" {LEFT 3}{DOWN}{down}{down};
            "#;

        let _ = eval(src);
    }
}
