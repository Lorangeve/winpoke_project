use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::SendInput;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SendMessageW, WM_CHAR, WM_COMMAND, WM_KEYDOWN, WM_KEYUP,
};

use crate::prelude::Result;

#[derive(Debug, Default)]
pub struct Message {
    pub msg: WindowMessage,
    pub count: u32,
}

#[derive(Debug)]
pub enum WindowMessage {
    KeyDown(u32),
    KeyUp(u32),
    Char(char),
    MouseMove(i32, i32),
    Command(u32),
    Input(InputSequence),
}

impl Default for WindowMessage {
    fn default() -> Self {
        WindowMessage::KeyDown(0)
    }
}

pub type InputSequence = Vec<InputMessage>;

#[derive(Debug)]
pub enum InputMessage {
    Mouse { x: i32, y: i32 },
    Keyboard { key: u32, flag: Option<u32> },
    Hardware,
}

pub(crate) fn send_message(
    hwnd: HWND,
    msg: u32,
    wparam: Option<u32>,
    lparam: Option<u32>,
    count: u32,
) -> Result<()> {
    for _ in 0..count.max(1) {
        let wparam = wparam.map(|w| WPARAM(w as usize));
        let lparam = lparam.map(|l| LPARAM(l as isize));

        unsafe { SendMessageW(hwnd, msg, wparam, lparam) };
    }

    Ok(())
}

pub(crate) fn send_message_seq(hwnd: HWND, msg_seq: Vec<Message>) -> Result<()> {
    for message in msg_seq {
        match message.msg {
            WindowMessage::Char(c) => {
                send_message(hwnd, WM_CHAR, Some(c as _), None, message.count)?
            }
            WindowMessage::Command(cmd) => send_message(
                hwnd,
                WM_COMMAND,
                Some(cmd),
                Some(hwnd.0 as _),
                message.count,
            )?,
            WindowMessage::KeyDown(virtual_key) => {
                send_message(hwnd, WM_KEYDOWN, Some(virtual_key), None, message.count)?
            }
            WindowMessage::KeyUp(virtual_key) => {
                send_message(hwnd, WM_KEYUP, Some(virtual_key), None, message.count)?
            }
            WindowMessage::MouseMove(_, _) => todo!(),
            WindowMessage::Input(input_messages) => send_input_seq(input_messages)?,
        }
    }

    Ok(())
}

pub(crate) fn send_input_seq(input_seq: InputSequence) -> Result<()> {
    let cbsize = std::mem::size_of::<INPUT>();

    let mut inputs: Vec<INPUT> = Vec::new();

    for input in input_seq {
        match input {
            InputMessage::Mouse { x, y } => todo!(),
            InputMessage::Keyboard { key, flag } => {
                inputs.push(INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(key as _),
                            dwFlags: KEYBD_EVENT_FLAGS(flag.unwrap_or_default()),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            }
            InputMessage::Hardware => todo!(),
        }
    }

    unsafe { SendInput(inputs.as_ref(), cbsize as _) };

    Ok(())
}

#[cfg(test)]
mod tests {
    use windows::Win32::UI::Input::KeyboardAndMouse::{KEYEVENTF_KEYUP, VK_LEFT, VK_LWIN};

    use super::*;
    use crate::{error::Error, window::WindowInfo};

    #[test]
    fn test_send_message() -> Result<()> {
        let windows = WindowInfo::find_by_class_name("RegEdit_RegEdit") // "RegEdit_RegEdit"
            .expect("找不到指定窗口");
        let window = windows.into_iter().next().expect("没有窗口信息");
        window.set_foreground_window()?;

        let tree_wnd = window
            .find_child_windows()?
            .into_iter()
            .filter(|w| w.class_name == "SysTreeView32")
            .next()
            .ok_or(Error::WindowNotFound)?;

        tree_wnd.show_window()?;
        tree_wnd.set_focus()?;

        tree_wnd
            .send_message_seq(dbg!(vec![
                // 循环发送左箭头，折叠到根节点
                Message {
                    msg: WindowMessage::KeyDown(VK_LEFT.0 as _),
                    count: 5,
                },
                Message {
                    msg: WindowMessage::KeyDown(VK_LWIN.0 as _),
                    ..Default::default()
                },
                Message {
                    msg: WindowMessage::KeyDown('D' as _),
                    ..Default::default()
                },
                Message {
                    msg: WindowMessage::KeyUp(VK_LWIN.0 as _),
                    ..Default::default()
                },
                Message {
                    msg: WindowMessage::KeyUp('D' as _),
                    ..Default::default()
                },
            ]))
            .expect("发送消息失败");

        Ok(())
    }

    #[test]
    fn test_send_input() -> Result<()> {
        send_input_seq(vec![
            InputMessage::Keyboard {
                key: VK_LWIN.0 as _,
                flag: None,
            },
            InputMessage::Keyboard {
                key: 'D' as _,
                flag: None,
            },
            InputMessage::Keyboard {
                key: VK_LWIN.0 as _,
                flag: Some(KEYEVENTF_KEYUP.0),
            },
            InputMessage::Keyboard {
                key: 'D' as _,
                flag: Some(KEYEVENTF_KEYUP.0),
            },
        ])?;

        Ok(())
    }
}
