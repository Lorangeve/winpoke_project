pub(crate) mod map;

use std::fmt::Display;

use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WINDOW_STYLE, WS_MAXIMIZE};

use crate::window::style::map::STYLE_MAP;

#[derive(Debug, Default)]
pub struct WindowStyle {
    pub style: WINDOW_STYLE,
    pub extend_style: WINDOW_EX_STYLE,
}

impl Display for WindowStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut styles = Vec::new();

        // 解析样式（先检查组合样式，再检查单个样式）
        for &(value, name) in STYLE_MAP {
            if (self.style & WINDOW_STYLE(value)) == WINDOW_STYLE(value) && value != 0 {
                styles.push(name);
            }
        }

        for &(value, name) in map::EX_STYLE_MAP {
            if (self.extend_style & WINDOW_EX_STYLE(value)) == WINDOW_EX_STYLE(value) && value != 0
            {
                styles.push(name);
            }
        }

        write!(f, "{}", styles.join(" | "))
    }
}

impl WindowStyle {
    /// 是否最大化
    pub fn is_maximized(&self) -> bool {
        (self.style & WS_MAXIMIZE) == WS_MAXIMIZE // WS_MAXIMIZE
    }
}

#[cfg(test)]
mod tests {
    use crate::window::info::get_window_info;

    use windows::{Win32::UI::WindowsAndMessaging::FindWindowW, core::HSTRING};

    #[test]
    fn window_style() {
        let hwnd = unsafe { FindWindowW(&HSTRING::from("RegEdit_RegEdit"), None) }
            .expect("找不到指定窗口");
        let info = get_window_info(hwnd).expect("获取窗口信息失败");

        dbg!(info.style.to_string());
    }

    #[test]
    fn is_not_maximized() {
        let hwnd = unsafe { FindWindowW(&HSTRING::from("RegEdit_RegEdit"), None) }
            .expect("找不到指定窗口");
        let info = get_window_info(hwnd).expect("获取窗口信息失败");

        assert!(!info.style.is_maximized());
    }
}
