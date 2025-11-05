use std::ptr::NonNull;

use crate::{monitor::MonitorInfo, prelude::Result};

use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, EnumDisplayMonitors, GetDeviceCaps, GetMonitorInfoW, HDC, HMONITOR, LOGPIXELSX,
    LOGPIXELSY, MONITORINFO, MONITORINFOEXW,
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CMONITORS};
use windows::core::{BOOL, HSTRING};

/// 枚举所有显示器
pub fn enum_monitors() -> Result<Vec<MonitorInfo>> {
    let mut monitors = Vec::new();

    let lparam = LPARAM(NonNull::from_mut(&mut monitors).as_ptr() as _);

    unsafe { EnumDisplayMonitors(None, None, Some(monitor_enum_proc), lparam).ok() }?;

    Ok(monitors)
}

/// 显示器枚举回调函数
extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
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

    let hdc = unsafe { CreateDCW(&HSTRING::from(&device_name), None, None, None) };

    monitors.push(MonitorInfo {
        hmonitor,
        hdc,
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

/// 获取指定显示器的DPI
pub(crate) fn get_dpi_for_monitor(monitor: &MonitorInfo) -> Result<(u32, u32)> {
    let mut dpix = 0u32;
    let mut dpiy = 0u32;

    if cfg!(feature = "compatible") {
        dpix = unsafe { GetDeviceCaps(Some(monitor.hdc), LOGPIXELSX) } as _;
        dpiy = unsafe { GetDeviceCaps(Some(monitor.hdc), LOGPIXELSY) } as _;
    } else {
        unsafe { GetDpiForMonitor(monitor.hmonitor, MDT_EFFECTIVE_DPI, &mut dpix, &mut dpiy)? };
    }

    Ok((dpix as u32, dpiy as u32))
}

/// 获取当前系统的显示器数量
pub(crate) fn get_monitor_count() -> i32 {
    unsafe { GetSystemMetrics(SM_CMONITORS) }
}

pub(crate) fn monitor_from_point(x: i32, y: i32) -> Result<MonitorInfo> {
    let monitors = enum_monitors()?;

    for monitor in monitors {
        let (top, right, bottom, left) = monitor.rect;

        if x >= left && x <= right && y >= top && y <= bottom {
            return Ok(monitor);
        }
    }

    Err(crate::prelude::Error::NotFoundMonitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_monitor_count() {
        let count = get_monitor_count();
        println!("Monitor count: {}", count);
        assert!(count > 0);
    }

    #[test]
    fn test_enum_display_monitors() {
        let _ = dbg!(enum_monitors());
    }

    #[test]
    fn test_get_dpi_for_monitor() {
        let monitors = dbg!(enum_monitors()).expect("枚举显示器错误");

        monitors.iter().for_each(|mo| {
            let (dpix, dpiy) = get_dpi_for_monitor(mo).expect("获取显示器 DPI 错误");

            assert!(dpix != 0 && dpiy != 0);
        });
    }
}
