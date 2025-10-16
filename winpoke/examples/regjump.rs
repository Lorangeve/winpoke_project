use std::env;
use std::time::Duration;

use windows::Win32::UI::{
    Input::KeyboardAndMouse::{SetFocus, VK_LEFT, VK_RIGHT},
    WindowsAndMessaging::{SW_SHOW, SetForegroundWindow, ShowWindow},
};
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

    let mut count = 0;
    let window = loop {
        match find_regedit() {
            Ok(window) => break window,
            Err(_) if count == 3 => {
                create_process("C:\\Windows\\regedit.exe")?;
            }
            Err(_) => {
                std::thread::sleep(Duration::from_millis(500));
                println!("第{count}次重试...");
            }
        };
        count += 1;
    };

    // let handle = open_process(window.pid)?;

    println!("找到 regedit.exe 窗口 {:?}", window);

    if unsafe { ShowWindow(window.hwnd, SW_SHOW) }.as_bool() {
        println!("将 regedit 显示为前台窗口");
    }

    let tree_wnd = window
        .get_child_windows()?
        .into_iter()
        .filter(|w| w.class_name == "SysTreeView32")
        .next()
        .ok_or(Error::WindowNotFound)?;

    #[allow(unused_must_use)]
    unsafe {
        SetForegroundWindow(tree_wnd.hwnd);
        SetFocus(Some(tree_wnd.hwnd));

        window.send_message_seq(vec![
            Message {
                msg: WindowMessage::Command(0x10288),
                ..Default::default()
            },
            // 循环发送左箭头，折叠到根节点
            Message {
                msg: WindowMessage::KeyDown(VK_LEFT.0 as _),
                count: 30,
            },
        ]);

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
            };
            // println!("{c}");
        }
    };

    Ok(())
}

fn main() -> Result<()> {
    let target_path = env::args()
        .nth(1)
        .unwrap_or(r#"HKEY_CURRENT_USER\Software\Microsoft"#.to_string());

    active_regedit_by_path(target_path)?;

    Ok(())
}
