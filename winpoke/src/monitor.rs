use windows::Win32::Graphics::Gdi::HMONITOR;

pub(crate) mod info;

#[derive(Debug, Default)]
pub struct MonitorInfo {
    pub(crate) hmonitor: HMONITOR,
    pub width: i32,
    pub height: i32,
    pub scale: f32,
}

impl MonitorInfo {
    pub fn new() -> Self {
        info::get_monitor_pixel().map_or(MonitorInfo::default(), |(width, height)| Self {
            width,
            height,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_info() {
        let monitor_info = MonitorInfo::new();
        dbg!(&monitor_info);
        assert!(monitor_info.width > 0 && monitor_info.height > 0);
    }
}
