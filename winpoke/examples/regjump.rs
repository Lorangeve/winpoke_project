use std::env;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{VK_LEFT, VK_RIGHT};
use winpoke::{
    prelude::*,
    window::{
        active::create_process,
        msg::{Message, WindowMessage},
    },
};

fn find_regedit() -> Result<WindowInfo> {
    let windows = WindowInfo::find_by_class_name("RegEdit_RegEdit")?;

    windows.into_iter().next().ok_or(Error::WindowNotFound)
}

fn active_regedit_by_path(target_path: impl AsRef<str>) -> Result<()> {
    let _value_name = "";

    let window = find_regedit().or_else(|_| {
        create_process("C:\\Windows\\regedit.exe")?;
        std::thread::sleep(Duration::from_secs(1));
        find_regedit()
    })?;

    // let handle = open_process(window.pid)?;

    println!("找到 regedit.exe 窗口: \n{:?}", window);

    let tree_wnd = window
        .find_child_windows_with_class_name("SysTreeView32")?
        .into_iter()
        .next()
        .ok_or(Error::WindowNotFound)?;

    #[allow(unused_must_use)]
    tree_wnd.set_foreground_window()?;
    // tree_wnd.set_focus();

    window.send_message_seq(vec![
        Message {
            // 0x10288 是 regedit 的“定位到地址栏”命令（WM_COMMAND, ID_EDIT_JUMPTOADDRESSBAR）x10288),
            msg: WindowMessage::Command(0x10288),
            ..Default::default()
        },
        // 循环发送左箭头，折叠到根节点
        Message {
            msg: WindowMessage::KeyDown(VK_LEFT.0 as _),
            count: 30,
        },
    ])?;

    for c in target_path.as_ref().chars() {
        match c {
            '\\' => tree_wnd.send_message(Message {
                msg: WindowMessage::KeyDown(VK_RIGHT.0 as _),
                ..Default::default()
            }),
            _ => tree_wnd.send_message(Message {
                msg: WindowMessage::Char(c),
                ..Default::default()
            }),
        }?;
        // println!("{c}");
    }

    Ok(())
}

fn main() -> Result<()> {
    let target_path = env::args()
        .nth(1)
        .unwrap_or(r#"HKEY_CURRENT_USER\Software\Microsoft"#.to_string());

    active_regedit_by_path(target_path)?;

    Ok(())
}
