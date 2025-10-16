use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, WM_CHAR, WM_COMMAND, WM_KEYDOWN};

use crate::prelude::Result;

#[derive(Debug, Default)]
pub struct Message {
    pub msg: WindowMessage,
    pub count: u32,
}

#[derive(Debug)]
pub enum WindowMessage {
    KeyDown(u32),
    MouseMove(i32, i32),
    Char(char),
    Command(u32),
}

impl Default for WindowMessage {
    fn default() -> Self {
        WindowMessage::KeyDown(0)
    }
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
            WindowMessage::MouseMove(_, _) => todo!(),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Error, window::WindowInfo};

    use windows::Win32::UI::Input::KeyboardAndMouse::VK_LEFT;

    #[test]
    fn test_send_message() -> Result<()> {
        let windows = WindowInfo::find_by_class_name("RegEdit_RegEdit") // "RegEdit_RegEdit"
            .expect("找不到指定窗口");
        let window = windows.into_iter().next().expect("没有窗口信息");

        let tree_wnd = window
            .get_child_windows()?
            .into_iter()
            .filter(|w| w.class_name == "SysTreeView32")
            .next()
            .ok_or(Error::WindowNotFound)?;

        send_message_seq(
            tree_wnd.hwnd,
            dbg!(vec![
                // 循环发送左箭头，折叠到根节点
                Message {
                    msg: WindowMessage::KeyDown(VK_LEFT.0 as _),
                    count: 5,
                },
            ]),
        )
        .expect("发送消息失败");

        Ok(())
    }
}
