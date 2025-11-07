use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBD_EVENT_FLAGS, KEYBDINPUT,
    MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_VIRTUALDESK, MOUSEEVENTF_WHEEL,
    MOUSEINPUT, SendInput, VIRTUAL_KEY,
};

use crate::prelude::Result;

pub type InputSequence = Vec<Input>;

#[derive(Debug)]
pub enum Input {
    Mouse(MouseMessage),
    Keyboard { key: u32, flag: Option<u32> },
}

#[derive(Debug)]
pub enum MouseMessage {
    Move(i32, i32),
    MoveTo(i32, i32),
    LeftClick,
    RightClick,
    WheelScroll(i32),
}

impl Input {
    pub fn send_seq(input_seq: &InputSequence) -> Result<()> {
        let cbsize = std::mem::size_of::<INPUT>();

        let mut inputs: Vec<INPUT> = Vec::new();

        for input in input_seq {
            let input = match input {
                Input::Mouse(mouse_operate) => match mouse_operate {
                    MouseMessage::Move(x, y) => INPUT {
                        r#type: INPUT_MOUSE,
                        Anonymous: INPUT_0 {
                            mi: MOUSEINPUT {
                                dx: *x,
                                dy: *y,
                                mouseData: 0,
                                dwFlags: MOUSEEVENTF_MOVE,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                    MouseMessage::MoveTo(x, y) => INPUT {
                        r#type: INPUT_MOUSE,
                        Anonymous: INPUT_0 {
                            mi: MOUSEINPUT {
                                dx: *x,
                                dy: *y,
                                mouseData: 0,
                                dwFlags: MOUSEEVENTF_MOVE
                                    | MOUSEEVENTF_ABSOLUTE
                                    | MOUSEEVENTF_VIRTUALDESK,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                    MouseMessage::LeftClick => {
                        inputs.push(INPUT {
                            r#type: INPUT_MOUSE,
                            Anonymous: INPUT_0 {
                                mi: MOUSEINPUT {
                                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                                    ..Default::default()
                                },
                            },
                        });
                        INPUT {
                            r#type: INPUT_MOUSE,
                            Anonymous: INPUT_0 {
                                mi: MOUSEINPUT {
                                    dwFlags: MOUSEEVENTF_LEFTUP,
                                    ..Default::default()
                                },
                            },
                        }
                    }
                    MouseMessage::RightClick => {
                        inputs.push(INPUT {
                            r#type: INPUT_MOUSE,
                            Anonymous: INPUT_0 {
                                mi: MOUSEINPUT {
                                    dwFlags: MOUSEEVENTF_RIGHTDOWN,
                                    ..Default::default()
                                },
                            },
                        });
                        INPUT {
                            r#type: INPUT_MOUSE,
                            Anonymous: INPUT_0 {
                                mi: MOUSEINPUT {
                                    dwFlags: MOUSEEVENTF_RIGHTUP,
                                    ..Default::default()
                                },
                            },
                        }
                    }
                    MouseMessage::WheelScroll(r) => INPUT {
                        r#type: INPUT_MOUSE,
                        Anonymous: INPUT_0 {
                            mi: MOUSEINPUT {
                                mouseData: *r as _,
                                dwFlags: MOUSEEVENTF_WHEEL,
                                ..Default::default()
                            },
                        },
                    },
                },
                Input::Keyboard { key, flag } => INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(*key as _),
                            dwFlags: KEYBD_EVENT_FLAGS(flag.unwrap_or_default()),
                            ..Default::default()
                        },
                    },
                },
            };

            inputs.push(input);
        }

        unsafe { SendInput(inputs.as_ref(), cbsize as _) };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Keyboard, Result};

    use windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_KEYUP;

    #[test]
    fn test_send_input() -> Result<()> {
        Input::send_seq(&vec![
            Input::Keyboard {
                key: Keyboard::LWin.to_virtual_key(),
                flag: None,
            },
            Input::Keyboard {
                key: Keyboard::Char('D').to_virtual_key(),
                flag: None,
            },
            Input::Keyboard {
                key: Keyboard::LWin.to_virtual_key(),
                flag: Some(KEYEVENTF_KEYUP.0),
            },
            Input::Keyboard {
                key: Keyboard::Char('D').to_virtual_key(),
                flag: Some(KEYEVENTF_KEYUP.0),
            },
        ])?;

        Ok(())
    }

    #[test]
    fn test_send_mouse_input() -> Result<()> {
        Input::send_seq(&vec![
            // 移动鼠标到屏幕中心
            Input::Mouse(MouseMessage::MoveTo(65535 / 2, 65535 / 2)),
            //右键单击
            Input::Mouse(MouseMessage::RightClick),
            // 左键单击
            Input::Mouse(MouseMessage::LeftClick),
            // 滚动鼠标滚轮
            Input::Mouse(MouseMessage::WheelScroll(-120)),
        ])?;

        Ok(())
    }
}
