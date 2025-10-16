use std::ptr::NonNull;

use crate::error::Error;
use crate::prelude::Result;
use crate::window::style::WindowStyle;

use super::*;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowExW, GetClassNameW, GetWindowInfo, GetWindowTextW,
    GetWindowThreadProcessId, WINDOWINFO, WS_ACTIVECAPTION,
};
use windows::core::{BOOL, HSTRING};

/// 安全枚举所有顶级窗口并返回窗口句柄列表
pub(crate) fn enumerate_top_level_windows() -> Result<Vec<HWND>> {
    let mut hwnds = Vec::new();

    // 将Vec的NonNull指针转换为LPARAM传递给回调
    let lparam = LPARAM(NonNull::from_mut(&mut hwnds).as_ptr() as isize);

    // 调用Windows API枚举窗口（unsafe：与C API交互）
    unsafe { EnumWindows(Some(enum_func), lparam) }.map_err(|_| Error::EnumWindowsFailed)?;

    Ok(hwnds)
}

/// 窗口枚举回调函数（unsafe：C调用约定，直接操作原始指针）
extern "system" fn enum_func(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // 将LPARAM转换回NonNull<Vec<HWND>>（安全性：由调用方保证指针有效）
    let hwnds_ptr = match NonNull::new(lparam.0 as *mut Vec<HWND>) {
        Some(ptr) => ptr,
        None => return BOOL(0), // 指针无效时终止枚举
    };

    // 向Vec中添加窗口句柄（安全性：调用方保证指针指向有效的Vec且生命周期正确）
    unsafe { (*hwnds_ptr.as_ptr()).push(hwnd) };

    BOOL(1) // 继续枚举
}

pub(crate) fn enum_child_window(parent: HWND) -> Result<Vec<HWND>> {
    let mut pre_child = unsafe { FindWindowExW(Some(parent), None, None, None) }
        .map_err(|_| Error::WindowNotFound)?;
    let mut child_windows = vec![pre_child];

    while let Ok(child) = unsafe { FindWindowExW(Some(parent), Some(pre_child), None, None) }
        && !child.is_invalid()
    {
        child_windows.push(child);

        pre_child = child;
    }

    Ok(child_windows)
}

pub(crate) fn enum_child_window_with_class_name(
    parent: HWND,
    class_name: impl AsRef<str>,
) -> Result<Vec<HWND>> {
    let mut pre_child = unsafe {
        FindWindowExW(
            Some(parent),
            None,
            &HSTRING::from(class_name.as_ref()),
            None,
        )
    }
    .map_err(|_| Error::WindowNotFound)?;
    let mut child_windows = vec![pre_child];

    while let Ok(child) = unsafe { FindWindowExW(Some(parent), Some(pre_child), None, None) }
        && !child.is_invalid()
    {
        child_windows.push(child);

        pre_child = child;
    }

    Ok(child_windows)
}

/// 通过窗口句柄 [`HWND`] 获取指定窗口的 `tid` 和 `pid`
pub(crate) fn get_window_tid_and_pid(hwnd: HWND) -> Result<(u32, u32)> {
    let mut pid: u32 = 0;

    let tid = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };

    Ok((tid, pid))
}

/// 通过窗口句柄获取窗口标题
pub(crate) fn get_window_caption(hwnd: HWND) -> Result<String> {
    let mut buffer = [0u16; 255];

    let len = unsafe { GetWindowTextW(hwnd, &mut buffer) };

    Ok(String::from_utf16_lossy(&buffer[..len as usize]))
}

/// 通过窗口句柄获取窗口类名
pub(crate) fn get_window_class_name(hwnd: HWND) -> Result<String> {
    let mut buffer = [0u16; 512];
    let len = unsafe { GetClassNameW(hwnd, &mut buffer) };

    Ok(String::from_utf16_lossy(&buffer[..len as usize]))
}

/// 通过窗口句柄获取窗口信息
pub(crate) fn get_window_info(hwnd: HWND) -> Result<WindowInfo> {
    let mut info = WINDOWINFO {
        cbSize: std::mem::size_of::<WINDOWINFO>() as u32,
        ..Default::default()
    };
    unsafe { GetWindowInfo(hwnd, &mut info) }.map_err(|_| Error::GetWindowInfoFailed)?;

    let WINDOWINFO {
        #[allow(unused_variables)]
        cbSize,
        #[allow(unused_variables)]
        atomWindowType,
        #[allow(unused_variables)]
        wCreatorVersion,

        rcWindow,
        rcClient,
        dwStyle,
        dwExStyle,
        dwWindowStatus,
        cxWindowBorders,
        cyWindowBorders,
    } = info;

    let (tid, pid) = get_window_tid_and_pid(hwnd)?;
    let caption = get_window_caption(hwnd)?;
    let class_name = get_window_class_name(hwnd)?;
    let position = (rcWindow.top, rcWindow.right, rcWindow.bottom, rcWindow.left);
    let client_position = (rcClient.top, rcClient.right, rcClient.bottom, rcClient.left);
    let is_active = dwWindowStatus == WS_ACTIVECAPTION.0;
    let border = (cxWindowBorders, cyWindowBorders);
    let style = WindowStyle {
        style: dwStyle,
        extend_style: dwExStyle,
    };

    Ok(WindowInfo {
        hwnd,
        caption,
        class_name,
        pid,
        tid,
        position,
        client_position,
        border,
        is_active,
        style,
    })
}

#[cfg(test)]
mod tests {
    use windows::{Win32::UI::WindowsAndMessaging::FindWindowW, core::HSTRING};

    use super::*;

    #[test]
    fn test_print_window_info() {
        let hwnd = unsafe { FindWindowW(&HSTRING::from("RegEdit_RegEdit"), None) }
            .expect("找不到指定窗口");
        let info = get_window_info(hwnd).expect("获取窗口信息失败");
        dbg!(info);
    }

    #[test]
    fn test_enumerate_windows() {
        let hwnds = dbg!(enumerate_top_level_windows());

        for hwnd in hwnds.unwrap().into_iter() {
            let info = get_window_info(hwnd).expect("获取窗口信息失败");
            println!("{info:?}");
        }
    }

    #[test]
    fn test_print_all_child_windows() {
        let hwnd = unsafe { FindWindowW(&HSTRING::from("RegEdit_RegEdit"), None) }
            .expect("找不到指定窗口");
        let childs = enum_child_window(hwnd).expect("查找子窗口失败");
        for child in childs {
            let info = get_window_info(child).expect("获取窗口信息失败");
            println!("{info:#?}");
        }
    }
}
