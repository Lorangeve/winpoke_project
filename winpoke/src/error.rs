use thiserror::Error;
use windows::core;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Windows API 调用失败: {0}")]
    WindowsError(#[from] core::Error),
    #[error("找不到指定窗口")]
    WindowNotFound,
    #[error("显示窗口失败")]
    ShowWindowFailed,
    #[error("设置前台窗口失败")]
    SetForegroundWindowFailed,
    #[error("设置窗口焦点失败")]
    SetFocusFailed(core::Error),
    #[error("获取窗口信息失败")]
    GetWindowInfoFailed,
    #[error("枚举窗口失败")]
    EnumWindowsFailed,
}
