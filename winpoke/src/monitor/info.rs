use std::ptr::NonNull;

use crate::{monitor::MonitorInfo, prelude::Result};

use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, RECT},
        Graphics::Gdi::{
            EnumDisplayMonitors, GetDC, GetDeviceCaps, GetMonitorInfoW, HDC, HMONITOR, LOGPIXELSX,
            MONITORINFO, MONITORINFOEXW, ReleaseDC,
        },
        UI::{
            HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI, MONITOR_DPI_TYPE},
            WindowsAndMessaging::{GetSystemMetrics, SM_CMONITORS, SM_CXSCREEN, SM_CYSCREEN},
        },
    },
    core::BOOL,
};

/// 枚举所有显示器
pub fn enum_monitors() -> Result<Vec<MonitorInfo>> {
    let mut monitors = Vec::new();

    let lparam = LPARAM(NonNull::from_mut(&mut monitors).as_ptr() as _);

    unsafe {
        EnumDisplayMonitors(Some(HDC::default()), None, Some(monitor_enum_proc), lparam).ok()
    }?;

    Ok(monitors)
}

/// 显示器枚举回调函数
extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    dbg!(hmonitor);
    dbg!(hdc);

    let monitors = match NonNull::new(lparam.0 as *mut Vec<MonitorInfo>) {
        Some(ptr) => unsafe { &mut *ptr.as_ptr() },
        None => return BOOL(0),
    };

    #[allow(non_snake_case)]
    let monitorInfo = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFOEXW>() as _,
        ..Default::default()
    };

    let monitorinfo_ex: *mut MONITORINFOEXW = &mut MONITORINFOEXW {
        monitorInfo,
        ..Default::default()
    };

    unsafe { GetMonitorInfoW(hmonitor, monitorinfo_ex as *mut _) }.expect("GetMonitorInfoW failed");

    let MONITORINFOEXW {
        monitorInfo: monitorinfo,
        szDevice,
    } = unsafe { &*monitorinfo_ex };

    let MONITORINFO {
        cbSize: _,
        rcMonitor,
        rcWork: _,
        dwFlags: _,
    } = monitorinfo;

    let device_name = String::from_utf16_lossy(szDevice)
        .trim_end_matches("\0")
        .to_string();

    monitors.push(MonitorInfo {
        hmonitor,
        device_name,
        number: monitors.len() + 1,
        rect: (
            rcMonitor.top,
            rcMonitor.right,
            rcMonitor.bottom,
            rcMonitor.left,
        ),
        ..Default::default()
    });

    BOOL(1)
}

/// 获取当前显视器的像素数
/// 返回元组 (x, y)
pub(crate) fn get_monitor_pixel() -> Option<(i32, i32)> {
    let screen_x = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_y = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    Some((screen_x, screen_y))
}

/// 当前显示器的中间点坐标
/// 返回元组 (x, y)
pub(crate) fn get_monitor_center() -> Option<(i32, i32)> {
    get_monitor_pixel().map(|(x, y)| (x / 2, y / 2))
}

// 获取当前系统的显示器数量
pub(crate) fn get_monitor_count() -> i32 {
    unsafe { GetSystemMetrics(SM_CMONITORS) }
}

/// 获取主显示器的DPI
fn get_primary_screen_dpi() -> Result<f32> {
    let dpi = unsafe {
        let hwnd = HWND::default();
        let hdc = GetDC(Some(hwnd));
        let dpi = GetDeviceCaps(Some(hdc), LOGPIXELSX);
        ReleaseDC(Some(hwnd), hdc);
        dpi
    };

    let x = unsafe {
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        GetDpiForMonitor(HMONITOR::default(), MDT_EFFECTIVE_DPI, &mut x, &mut y)?;

        x
    };

    Ok(x as f32 / dpi as f32 * 100.0)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_get_monitor_pixel() {
        let (x, y) = get_monitor_pixel().unwrap();
        println!("Monitor pixel: {} x {}", x, y);
        assert!(x > 0 && y > 0);
    }

    #[test]
    fn test_get_monitor_center() {
        let (x, y) = get_monitor_center().unwrap();
        println!("Monitor center: {} , {}", x, y);
        assert!(x > 0 && y > 0);
    }

    #[test]
    fn test_get_monitor_count() {
        let count = get_monitor_count();
        println!("Monitor count: {}", count);
        assert!(count > 0);
    }

    #[test]
    fn test_get_primary_screen_dpi() {
        let dpi = get_primary_screen_dpi().unwrap();
        println!("Primary screen DPI: {}", dpi);
        assert!(dpi > 0.0);
    }

    #[test]
    fn test_enum_display_monitors() {
        let _ = dbg!(enum_monitors());
    }
}
