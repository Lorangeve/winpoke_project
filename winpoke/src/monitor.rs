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
    pub scale: f32,
}

impl MonitorInfo {
    pub fn all_monitors() -> Result<Vec<MonitorInfo>> {
        info::enum_monitors()
    }

    pub fn is_primary(&self) -> bool {
        let (t, _, _, l) = self.rect;

        if t == 0 && l == 0 { true } else { false }
    }

    pub fn count() -> i32 {
        info::get_monitor_count()
    }
}

impl Drop for MonitorInfo {
    fn drop(&mut self) {
        unsafe {
            if !self.hdc.is_invalid() {
                let _ = DeleteDC(self.hdc);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_info() {
        // let monitor_info = MonitorInfo::new();
        // dbg!(&monitor_info);
        // assert!(monitor_info.width > 0 && monitor_info.height > 0);
    }

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
}
