use windows::Win32::Graphics::Gdi::HMONITOR;

pub(crate) mod info;

#[derive(Debug, Default)]
pub struct MonitorInfo {
    pub(crate) hmonitor: HMONITOR,
    pub device_name: String,
    ///显示器矩形坐标（上，右，下，左）
    pub rect: (i32, i32, i32, i32),
    pub scale: f32,
}

impl MonitorInfo {
    pub fn new() -> Self {
        todo!()
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
}
