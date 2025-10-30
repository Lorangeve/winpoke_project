//! 自定义错误类型，用于处理 Windows 窗口操作中的各种错误情况。
//! 错误名均为「谓-宾」结构，以清晰表达错误的含义。
use thiserror::Error;
use windows::core;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Windows API 调用失败: {0}")]
    WindowsError(#[from] core::Error),
    #[error("找不到指定窗口")]
    FoundWindowError,
    #[error("显示窗口失败")]
    ShowWindowError,
    #[error("设置前台窗口失败")]
    SetForegroundWindowError,
    #[error("设置窗口焦点失败")]
    SetFocusError(core::Error),
    #[error("获取窗口信息失败")]
    GetWindowInfoError,
    #[error("枚举窗口失败")]
    EnumWindowsError(core::Error),
}
