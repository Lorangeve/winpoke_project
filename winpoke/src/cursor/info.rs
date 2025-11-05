use windows::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};

use crate::prelude::Result;

/// 获取当前鼠标光标位置，单位：像素
/// 返回值：(x, y)
pub(crate) fn get_cursor_pos() -> Result<(i32, i32)> {
    let mut point = POINT::default();

    unsafe { GetCursorPos(&mut point)? };

    Ok((point.x, point.y))
}
