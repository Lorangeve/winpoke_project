pub mod keyboard;
pub mod mouse;

use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::SendInput;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBD_EVENT_FLAGS, KEYBDINPUT, MOUSEEVENTF_MOVE,
    MOUSEINPUT, VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, SendMessageW, WM_CHAR, WM_COMMAND, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN,
    WM_LBUTTONUP, WM_MOUSEMOVE,
};

use crate::prelude::Result;

#[derive(Debug, Default)]
pub struct Message {
    pub msg: WindowMessage,
    pub count: u32,
}

#[derive(Debug)]
pub enum WindowMessage {
    /// 按下键盘按键，参数为虚拟键码
    KeyDown(u32),
    /// 松开键盘按键，参数为虚拟键码
    KeyUp(u32),
    /// 输入字符，参数为字符
    Char(char),
    /// 移动鼠标到指定坐标
    MouseMoveTo(i32, i32),
    /// 相对于当前位置移动鼠标
    MouseMoveBy(i32, i32),
    /// 单击鼠标左键，参数为坐标
    MouseClick(u32, u32),
    /// 双击鼠标左键，参数为坐标
    MouseDoubleClick(u32, u32),
    /// 发送命令消息，参数为命令ID
    Command(u32),
    /// 发送输入序列，参数为输入消息序列
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

pub(crate) fn send_message_seq(hwnd: HWND, msg_seq: &Vec<Message>) -> Result<()> {
    for message in msg_seq {
        match &message.msg {
            WindowMessage::Char(c) => {
                send_message(hwnd, WM_CHAR, Some(*c as _), None, message.count)?
            }
            WindowMessage::Command(cmd) => send_message(
                hwnd,
                WM_COMMAND,
                Some(*cmd),
                Some(hwnd.0 as _),
                message.count,
            )?,
            WindowMessage::KeyDown(virtual_key) => {
                send_message(hwnd, WM_KEYDOWN, Some(*virtual_key), None, message.count)?
            }
            WindowMessage::KeyUp(virtual_key) => {
                send_message(hwnd, WM_KEYUP, Some(*virtual_key), None, message.count)?
            }
            WindowMessage::MouseMoveTo(x, y) => {
                let lparam = ((*y as u32) << 16) | (*x as u32);
                send_message(hwnd, WM_MOUSEMOVE, None, Some(lparam), message.count)?;
            }
            WindowMessage::MouseMoveBy(x, y) => {
                // 获取当前鼠标位置
                let mut point = POINT::default();
                unsafe { GetCursorPos(&mut point) }?;

                let new_x = point.x + x;
                let new_y = point.y + y;
                let lparam = ((new_y as u32) << 16) | (new_x as u32);
                send_message(hwnd, WM_MOUSEMOVE, None, Some(lparam), message.count)?;
            }
            WindowMessage::MouseClick(x, y) => {
                let lparam = ((*y as u32) << 16) | (*x as u32);
                send_message(hwnd, WM_LBUTTONDOWN, None, Some(lparam), message.count)?;
                send_message(hwnd, WM_LBUTTONUP, None, Some(lparam), message.count)?;
            }
            WindowMessage::MouseDoubleClick(x, y) => {
                let lparam = ((*y as u32) << 16) | (*x as u32);
                send_message(hwnd, WM_LBUTTONDOWN, None, Some(lparam), message.count)?;
                send_message(hwnd, WM_LBUTTONUP, None, Some(lparam), message.count)?;
                send_message(hwnd, WM_LBUTTONDOWN, None, Some(lparam), message.count)?;
                send_message(hwnd, WM_LBUTTONUP, None, Some(lparam), message.count)?;
            }
            WindowMessage::Input(input_messages) => send_input_seq(input_messages)?,
        }
    }

    Ok(())
}

pub(crate) fn send_input_seq(input_seq: &InputSequence) -> Result<()> {
    let cbsize = std::mem::size_of::<INPUT>();

    let mut inputs: Vec<INPUT> = Vec::new();

    for input in input_seq {
        match input {
            InputMessage::Mouse { x, y } => {
                inputs.push(INPUT {
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
                });
            }
            InputMessage::Keyboard { key, flag } => {
                inputs.push(INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VIRTUAL_KEY(*key as _),
                            dwFlags: KEYBD_EVENT_FLAGS(flag.unwrap_or_default()),
                            ..Default::default()
                        },
                    },
                });
            }
        }
    }

    unsafe { SendInput(inputs.as_ref(), cbsize as _) };

    Ok(())
}

#[cfg(test)]
mod tests {
    use windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_KEYUP;

    use super::*;
    use crate::{error::Error, prelude::Keyboard, window::WindowInfo};

    #[test]
    fn test_send_message() -> Result<()> {
        let windows = WindowInfo::find_by_class_name("RegEdit_RegEdit") // "RegEdit_RegEdit"
            .expect("找不到指定窗口");
        let window = windows.into_iter().next().expect("没有窗口信息");
        window.set_foreground_window()?;

        let tree_wnd = window
            .child_windows()?
            .into_iter()
            .filter(|w| w.class_name == "SysTreeView32")
            .next()
            .ok_or(Error::NotFoundWindowError)?;

        tree_wnd.show_window()?;
        // tree_wnd.set_focus()?;

        tree_wnd
            .send_message_seq(dbg!(&vec![
                // 循环发送左箭头，折叠到根节点
                Message {
                    msg: WindowMessage::KeyDown(Keyboard::ArrowLeft.to_virtual_key()),
                    count: 5,
                },
                Message {
                    msg: WindowMessage::KeyDown(Keyboard::Win.to_virtual_key()),
                    ..Default::default()
                },
                Message {
                    msg: WindowMessage::KeyDown(Keyboard::Char('D').to_virtual_key()),
                    ..Default::default()
                },
                Message {
                    msg: WindowMessage::KeyUp(Keyboard::Win.to_virtual_key()),
                    ..Default::default()
                },
                Message {
                    msg: WindowMessage::KeyUp(Keyboard::Char('D').to_virtual_key()),
                    ..Default::default()
                },
            ]))
            .expect("发送消息失败");

        Ok(())
    }

    #[test]
    fn test_send_input() -> Result<()> {
        send_input_seq(&vec![
            InputMessage::Keyboard {
                key: Keyboard::LWin.to_virtual_key(),
                flag: None,
            },
            InputMessage::Keyboard {
                key: Keyboard::Char('D').to_virtual_key(),
                flag: None,
            },
            InputMessage::Keyboard {
                key: Keyboard::LWin.to_virtual_key(),
                flag: Some(KEYEVENTF_KEYUP.0),
            },
            InputMessage::Keyboard {
                key: Keyboard::Char('D').to_virtual_key(),
                flag: Some(KEYEVENTF_KEYUP.0),
            },
        ])?;

        Ok(())
    }

    #[test]
    /// 移动鼠标到屏幕中心
    fn test_send_mouse_input() -> Result<()> {
        use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
        use windows::Win32::UI::WindowsAndMessaging::{SM_CXSCREEN, SM_CYSCREEN};

        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

        let center_x = (screen_width / 2) * 65535 / screen_width;
        let center_y = (screen_height / 2) * 65535 / screen_height;

        send_input_seq(&vec![InputMessage::Mouse {
            x: center_x,
            y: center_y,
        }])?;

        Ok(())
    }
}
