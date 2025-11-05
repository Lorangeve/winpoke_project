#![doc = include_str!("..\\README.md")]

pub mod error;
pub mod monitor;
pub mod window;

#[cfg(feature = "eval")]
pub mod evaluate;

#[cfg(feature = "parser")]
pub mod parser;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::window::WindowInfo;
    pub use crate::window::msg::Message;
    pub use crate::window::msg::WindowMessage;
    pub use crate::window::msg::keyboard::Keyboard;

    pub type Result<T> = std::result::Result<T, Error>;
}
