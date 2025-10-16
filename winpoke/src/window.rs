pub mod active;
pub(crate) mod info;
pub mod msg;
pub(crate) mod style;

use windows::Win32::Foundation::HWND;

use crate::prelude::Result;
use crate::window::active::{open_process, set_focus, show_window, wait_for_input_idle};
use crate::window::msg::{Message, send_message_seq};
use crate::window::style::WindowStyle;
use info::*;

#[derive(Debug, Default)]
pub struct WindowInfo {
    /// 窗口句柄
    pub hwnd: HWND,

    /// 窗口标题
    pub caption: String,

    /// 窗口类名
    pub class_name: String,

    /// 进程ID
    pub pid: u32,

    /// 线程ID
    pub tid: u32,

    /// 窗口坐标(上,右,下,左)
    pub position: (i32, i32, i32, i32),

    /// 工作区坐标(上,右,下,左)
    pub client_position: (i32, i32, i32, i32),

    /// 窗口边框(宽,高)
    pub border: (u32, u32),

    /// 窗口是否为活动窗口
    pub is_active: bool,

    pub style: WindowStyle,
}

impl WindowInfo {
    /// 通过类名查找**顶层**窗口
    pub fn find_by_class_name<T: AsRef<str>>(class_name: T) -> Result<Vec<Self>> {
        let infos: Vec<WindowInfo> = enumerate_top_level_windows()?
            .into_iter()
            .filter(|&hwnd| {
                get_window_class_name(hwnd).is_ok_and(|name| name == class_name.as_ref())
            })
            .flat_map(get_window_info)
            .collect();

        Ok(infos)
    }

    /// 获取一级子窗口
    pub fn get_child_windows(&self) -> Result<Vec<WindowInfo>> {
        let infos: Vec<WindowInfo> = enum_child_window(self.hwnd)?
            .into_iter()
            .flat_map(get_window_info)
            .collect();

        Ok(infos)
    }

    pub fn get_child_windows_with_class_name(
        &self,
        class_name: impl AsRef<str>,
    ) -> Result<Vec<WindowInfo>> {
        let infos: Vec<WindowInfo> = enum_child_window_with_class_name(self.hwnd, class_name)?
            .into_iter()
            .flat_map(get_window_info)
            .collect();

        Ok(infos)
    }

    /// 显示窗口
    pub fn show_window(&self) -> Result<()> {
        show_window(self.hwnd)
    }

    /// 设置窗口为前台窗口并获取焦点
    pub fn set_focus(&self) -> Result<()> {
        set_focus(self.hwnd)
    }

    /// 发送消息到窗口
    pub fn send_message_seq(&self, msg_seq: Vec<Message>) -> Result<()> {
        wait_for_input_idle(open_process(self.pid)?, 500)?;

        send_message_seq(self.hwnd, msg_seq)?;

        Ok(())
    }

    /// 发送消息到窗口
    pub fn send_message(&self, msg: Message) -> Result<()> {
        wait_for_input_idle(open_process(self.pid)?, 500)?;

        send_message_seq(self.hwnd, vec![msg])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_must_use)]
    fn test_find_by_class_name() {
        dbg!(WindowInfo::find_by_class_name("RegEdit_RegEdit"));
    }

    #[test]
    fn test_get_child_windows() {
        let windows = WindowInfo::find_by_class_name("RegEdit_RegEdit").unwrap();
        for window in windows {
            println!("Window: {:?}", window);
            let children = window.get_child_windows().unwrap();
            for child in children {
                println!("  Child: {:?}", child);
            }
        }
    }

    #[test]
    fn test_get_child_windows_with_class_name() {
        let windows = WindowInfo::find_by_class_name("RegEdit_RegEdit").unwrap();
        for window in windows {
            println!("Window: {:?}", window);
            let children = window
                .get_child_windows_with_class_name("SysTreeView32")
                .unwrap();
            for child in children {
                println!("  Child: {:?}", child);
            }
        }
    }
}
