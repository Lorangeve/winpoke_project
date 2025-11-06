use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::{HDC, HMONITOR};

pub(crate) mod info;

use crate::cursor;
use crate::prelude::Result;

#[derive(Debug, Default)]
pub struct MonitorInfo {
    pub(crate) hmonitor: HMONITOR,
    pub(crate) hdc: HDC,
    pub number: u32,
    pub device_name: String,
    ///显示器矩形坐标（上，右，下，左）
    pub rect: (i32, i32, i32, i32),
}

impl MonitorInfo {
    /// 枚举所有显示器
    pub fn all_monitors() -> Result<Vec<MonitorInfo>> {
        info::enum_monitors()
    }

    /// 判断是否为主显示器
    pub fn is_primary(&self) -> bool {
        let (t, _, _, l) = self.rect;

        if t == 0 && l == 0 { true } else { false }
    }

    pub fn current_monitor(&self) -> Result<MonitorInfo> {
        let (cx, cy) = cursor::info::get_cursor_pos()?;
        info::monitor_from_point(cx, cy)
    }

    pub fn primary_monitor() -> Result<MonitorInfo> {
        let monitors = info::enum_monitors()?;
        for monitor in monitors {
            if monitor.is_primary() {
                return Ok(monitor);
            }
        }
        Err(crate::prelude::Error::NotFoundMonitor)
    }

    /// 获取显示器宽度，单位：像素
    pub fn width(&self) -> u32 {
        let (_, r, _, l) = self.rect;
        ((r - l) as f32 / self.scale_factor().unwrap_or((1.0, 1.0)).0)
            .ceil()
            .abs() as _
    }

    /// 获取显示器高度，单位：像素
    pub fn height(&self) -> u32 {
        let (t, _, b, _) = self.rect;
        ((t - b) as f32 / self.scale_factor().unwrap_or((1.0, 1.0)).1)
            .ceil()
            .abs() as _
    }

    /// 获取显示器DPI，单位：每英寸点数
    /// 使用 feature: compatible 时，使用 GetDeviceCaps 获取 DPI，适用于 Windows 7 及更早版本
    pub fn dpi(&self) -> Result<(u32, u32)> {
        info::get_dpi_for_monitor(self)
    }

    /// 获取显示器缩放比
    /// 返回值为 (scale_x, scale_y)
    pub fn scale_factor(&self) -> Result<(f32, f32)> {
        let (dpix, dpiy) = self.dpi()?;

        let scale_x = dpix as f32 / 96.0;
        let scale_y = dpiy as f32 / 96.0;

        Ok((scale_x, scale_y))
    }

    /// 规范化显示器矩形坐标左上角位置
    /// 返回值为 (normalized_x, normalized_y)
    pub fn absolute_left_top_postion(&self) -> (i32, i32) {
        let (t, _, _, l) = self.rect;

        let normalized_x = l * 65535 / self.width() as i32;
        let normalized_y = t * 65535 / self.height() as i32;

        (normalized_x, normalized_y)
    }

    /// 获取显示器绝对坐标缩放系数
    /// 返回值为 (factor_width, factor_height)
    pub fn absolute_factor(&self) -> (f32, f32) {
        let factor_width = 65535.0 / self.width() as f32;
        let factor_height = 65535.0 / self.height() as f32;

        (factor_width, factor_height)
    }

    /// 获取显示器中心点坐标
    pub fn center_point(&self) -> (i32, i32) {
        let (t, r, b, l) = self.rect;
        let center_x = (l + r) / 2;
        let center_y = (t + b) / 2;
        (center_x, center_y)
    }

    /// 获取系统显示器数量
    pub fn count() -> i32 {
        info::get_monitor_count()
    }
}

impl Drop for MonitorInfo {
    fn drop(&mut self) {
        unsafe {
            if !self.hdc.is_invalid() {
                DeleteDC(self.hdc).expect("HDC 无法释放");
            }
        }
    }
}

pub struct VirtualScreenInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl VirtualScreenInfo {
    pub fn new() -> Self {
        info::get_virtual_screen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_primary() {
        let monitors = MonitorInfo::all_monitors().unwrap();
        for monitor in monitors {
            if monitor.is_primary() {
                println!("Primary monitor: {:?}", monitor);
            } else {
                println!("Secondary monitor: {:?}", monitor);
            }
        }
    }

    #[test]
    fn test_scale_factor_and_width_height() {
        let monitors = MonitorInfo::all_monitors().unwrap();
        for monitor in monitors {
            let (scale_x, scale_y) = monitor.scale_factor().unwrap();
            println!(
                "Monitor {} scale factor: ({}, {})",
                monitor.number, scale_x, scale_y
            );
            assert!(scale_x > 0.0 && scale_y > 0.0);

            let width = monitor.width();
            let height = monitor.height();
            println!(
                "Monitor {} dimensions: {} x {}",
                monitor.number, width, height
            );
            assert!(width > 0 && height > 0);
        }
    }

    #[test]
    fn test_center_point() {
        let monitors = MonitorInfo::all_monitors().unwrap();
        for monitor in monitors {
            let (center_x, center_y) = monitor.center_point();
            println!(
                "Monitor {} center point: ({}, {})",
                monitor.number, center_x, center_y
            );
        }
    }

    #[test]
    fn test_absolute_posiotion() {
        let monitors = MonitorInfo::all_monitors().unwrap();
        for monitor in monitors {
            let abs_rect = monitor.absolute_left_top_postion();
            println!("Monitor {} absolute rect: {:?}", monitor.number, abs_rect);
        }
    }
}
