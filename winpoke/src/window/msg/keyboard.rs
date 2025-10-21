use std::str::FromStr;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    VK_BACK, VK_CAPITAL, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_HOME,
    VK_INSERT, VK_LEFT, VK_LWIN, VK_MENU, VK_NEXT, VK_NUMLOCK, VK_NUMPAD0, VK_PAUSE, VK_PRIOR,
    VK_RETURN, VK_RIGHT, VK_RWIN, VK_SCROLL, VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
};

pub enum Keyboard {
    /// 控制键：Ctrl
    Ctrl,
    /// 控制键：Alt
    Alt,
    /// 控制键：Shift
    Shift,

    /// Windows 键（任意 Win 键）
    Win,
    /// 左侧 Windows 键
    LWin,
    /// 右侧 Windows 键
    RWin,

    /// 方向键：上
    ArrowUp,
    /// 方向键：下
    ArrowDown,
    /// 方向键：左
    ArrowLeft,
    /// 方向键：右
    ArrowRight,

    /// Tab 键
    Tab,
    /// Enter/Return 键
    Enter,
    /// Esc 键
    Esc,
    /// 空格键
    Space,
    /// 退格键
    Backspace,
    /// 删除键
    Delete,
    /// 插入键
    Insert,
    /// Home 键
    Home,
    /// End 键
    End,
    /// Page Up 键
    PageUp,
    /// Page Down 键
    PageDown,

    /// NumLock 键
    NumLock,
    /// CapsLock 键
    CapsLock,
    /// ScrollLock 键
    ScrollLock,
    /// Pause 键
    Pause,

    /// 需要发送的字符，仅支持 ASCII 字符
    Char(char),

    /// 功能键, F1 - F24
    F(u16),

    /// 小键盘, Numpad0 - Numpad9
    Numpad(u16),
}

impl FromStr for Keyboard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ctrl" => Ok(Keyboard::Ctrl),
            "alt" => Ok(Keyboard::Alt),
            "shift" => Ok(Keyboard::Shift),
            "win" => Ok(Keyboard::Win),
            "lwin" => Ok(Keyboard::LWin),
            "rwin" => Ok(Keyboard::RWin),
            "up" | "arrowup" => Ok(Keyboard::ArrowUp),
            "down" | "arrowdown" => Ok(Keyboard::ArrowDown),
            "left" | "arrowleft" => Ok(Keyboard::ArrowLeft),
            "right" | "arrowright" => Ok(Keyboard::ArrowRight),
            "tab" => Ok(Keyboard::Tab),
            "enter" | "return" => Ok(Keyboard::Enter),
            "esc" | "escape" => Ok(Keyboard::Esc),
            "space" => Ok(Keyboard::Space),
            "backspace" => Ok(Keyboard::Backspace),
            "delete" | "del" => Ok(Keyboard::Delete),
            "insert" | "ins" => Ok(Keyboard::Insert),
            "home" => Ok(Keyboard::Home),
            "end" => Ok(Keyboard::End),
            "pageup" => Ok(Keyboard::PageUp),
            "pagedown" => Ok(Keyboard::PageDown),
            "numlock" => Ok(Keyboard::NumLock),
            "capslock" => Ok(Keyboard::CapsLock),
            "scrolllock" => Ok(Keyboard::ScrollLock),
            "pause" => Ok(Keyboard::Pause),
            _ => {
                // F1-F24
                if let Some(f_num) = s.strip_prefix('f') {
                    if let Ok(n) = f_num.parse::<u16>() {
                        if (1..=24).contains(&n) {
                            return Ok(Keyboard::F(n));
                        }
                    }
                }
                // Numpad0-Numpad9
                if let Some(np_num) = s.strip_prefix("numpad") {
                    if let Ok(n) = np_num.parse::<u16>() {
                        if (0..=9).contains(&n) {
                            return Ok(Keyboard::Numpad(n));
                        }
                    }
                }
                // 单字符
                if s.len() == 1 {
                    let c = s.chars().next().unwrap();
                    if c.is_ascii() {
                        return Ok(Keyboard::Char(c));
                    }
                }
                Err(())
            }
        }
    }
}

impl From<Keyboard> for u32 {
    fn from(value: Keyboard) -> Self {
        match value {
            Keyboard::Ctrl => VK_CONTROL.0 as u32,
            Keyboard::Alt => VK_MENU.0 as u32,
            Keyboard::Shift => VK_SHIFT.0 as u32,
            Keyboard::Win | Keyboard::LWin => VK_LWIN.0 as u32,
            Keyboard::RWin => VK_RWIN.0 as u32,

            // 方向键
            Keyboard::ArrowUp => VK_UP.0 as u32,
            Keyboard::ArrowDown => VK_DOWN.0 as u32,
            Keyboard::ArrowLeft => VK_LEFT.0 as u32,
            Keyboard::ArrowRight => VK_RIGHT.0 as u32,

            // 其他常用键
            Keyboard::Tab => VK_TAB.0 as u32,
            Keyboard::Enter => VK_RETURN.0 as u32,
            Keyboard::Esc => VK_ESCAPE.0 as u32,
            Keyboard::Space => VK_SPACE.0 as u32,
            Keyboard::Backspace => VK_BACK.0 as u32,
            Keyboard::Delete => VK_DELETE.0 as u32,
            Keyboard::Insert => VK_INSERT.0 as u32,
            Keyboard::Home => VK_HOME.0 as u32,
            Keyboard::End => VK_END.0 as u32,
            Keyboard::PageUp => VK_PRIOR.0 as u32,
            Keyboard::PageDown => VK_NEXT.0 as u32,

            Keyboard::Pause => VK_PAUSE.0 as u32,

            // 小键盘
            Keyboard::Numpad(n) if (0..=9).contains(&n) => (VK_NUMPAD0.0 + n) as u32,
            Keyboard::Numpad(_) => panic!("小键盘仅支持 0-9"),

            // 锁键
            Keyboard::NumLock => VK_NUMLOCK.0 as u32,
            Keyboard::CapsLock => VK_CAPITAL.0 as u32,
            Keyboard::ScrollLock => VK_SCROLL.0 as u32,

            // 功能键
            Keyboard::F(n) if (1..=24).contains(&n) => (VK_F1.0 - 1 + n) as u32,
            Keyboard::F(_) => panic!("功能键仅支持 F1-F24"),

            Keyboard::Char(c) if c.is_ascii() => c as u32,
            Keyboard::Char(_) => panic!("仅支持 ASCII 字符"),
        }
    }
}

impl Keyboard {
    /// 获取按键的虚拟键码
    pub fn to_virtual_key(&self) -> u32 {
        match self {
            Keyboard::Ctrl => VK_CONTROL.0 as u32,
            Keyboard::Alt => VK_MENU.0 as u32,
            Keyboard::Shift => VK_SHIFT.0 as u32,
            Keyboard::Win | Keyboard::LWin => VK_LWIN.0 as u32,
            Keyboard::RWin => VK_RWIN.0 as u32,

            // 方向键
            Keyboard::ArrowUp => VK_UP.0 as u32,
            Keyboard::ArrowDown => VK_DOWN.0 as u32,
            Keyboard::ArrowLeft => VK_LEFT.0 as u32,
            Keyboard::ArrowRight => VK_RIGHT.0 as u32,

            // 其他常用键
            Keyboard::Tab => VK_TAB.0 as u32,
            Keyboard::Enter => VK_RETURN.0 as u32,
            Keyboard::Esc => VK_ESCAPE.0 as u32,
            Keyboard::Space => VK_SPACE.0 as u32,
            Keyboard::Backspace => VK_BACK.0 as u32,
            Keyboard::Delete => VK_DELETE.0 as u32,
            Keyboard::Insert => VK_INSERT.0 as u32,
            Keyboard::Home => VK_HOME.0 as u32,
            Keyboard::End => VK_END.0 as u32,
            Keyboard::PageUp => VK_PRIOR.0 as u32,
            Keyboard::PageDown => VK_NEXT.0 as u32,

            Keyboard::Pause => VK_PAUSE.0 as u32,

            // 小键盘
            Keyboard::Numpad(n) if (0..=9).contains(n) => (VK_NUMPAD0.0 + n) as u32,
            Keyboard::Numpad(_) => panic!("小键盘仅支持 0-9"),

            // 锁键
            Keyboard::NumLock => VK_NUMLOCK.0 as u32,
            Keyboard::CapsLock => VK_CAPITAL.0 as u32,
            Keyboard::ScrollLock => VK_SCROLL.0 as u32,

            // 功能键
            Keyboard::F(n) if (1..=24).contains(n) => (VK_F1.0 - 1 + n) as u32,
            Keyboard::F(_) => panic!("功能键仅支持 F1-F24"),

            Keyboard::Char(c) if c.is_ascii() => *c as u32,
            Keyboard::Char(_) => panic!("仅支持 ASCII 字符"),
        }
    }
}
