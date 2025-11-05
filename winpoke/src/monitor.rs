use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::{HDC, HMONITOR};

pub(crate) mod info;

use crate::prelude::Result;

#[derive(Debug, Default)]
pub struct MonitorInfo {
    pub(crate) hmonitor: HMONITOR,
    pub(crate) hdc: HDC,
    pub number: usize,
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
}
