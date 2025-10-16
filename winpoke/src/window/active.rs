use windows::Win32::Foundation::HWND;
use windows::Win32::System::Threading::WaitForInputIdle;
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::{SW_SHOW, SetForegroundWindow, ShowWindow};
use windows::Win32::{
    Foundation::HANDLE,
    System::Threading::{
        CreateProcessW, OpenProcess, PROCESS_ALL_ACCESS, PROCESS_CREATION_FLAGS,
        PROCESS_INFORMATION, STARTUPINFOW,
    },
};
use windows::core::{HSTRING, PWSTR};

use crate::error::Error;
use crate::prelude::Result;

pub fn create_process(cmd: impl AsRef<str>) -> Result<HANDLE> {
    let regedit_path = HSTRING::from(cmd.as_ref());

    let startup_info = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };
    let mut process_info = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessW(
            None,
            Some(PWSTR(regedit_path.as_ptr() as *mut _)),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &startup_info,
            &mut process_info,
        )?
    };

    Ok(process_info.hProcess)
}

pub fn open_process(pid: u32) -> Result<HANDLE> {
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid)? };

    Ok(handle)
}

pub(crate) fn set_focus(hwnd: HWND) -> Result<()> {
    unsafe { SetForegroundWindow(hwnd) }
        .as_bool()
        .then_some(())
        .ok_or(Error::SetForegroundWindowFailed)?;

    unsafe { SetFocus(Some(hwnd)) }.map_err(|_| Error::SetFocusFailed)?;

    Ok(())
}

pub(crate) fn show_window(hwnd: HWND) -> Result<()> {
    unsafe { ShowWindow(hwnd, SW_SHOW) }
        .as_bool()
        .then_some(())
        .ok_or(Error::ShowWindowFailed)
}

pub fn wait_for_input_idle(handle: HANDLE, milliseconds: u32) -> Result<u32> {
    let result = unsafe { WaitForInputIdle(handle, milliseconds) };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::UI::WindowsAndMessaging::FindWindowW;
    use windows::core::HSTRING;

    #[test]
    fn test_open_process() {
        let hwnd = unsafe { FindWindowW(&HSTRING::from("RegEdit_RegEdit"), None) }
            .expect("找不到指定窗口");
        let info = crate::window::info::get_window_info(hwnd).expect("获取窗口信息失败");
        let handle = open_process(info.pid).expect("打开进程失败");
        println!("process handle: {:?}", handle);
    }
}
