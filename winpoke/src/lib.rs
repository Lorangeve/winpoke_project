pub mod error;
pub mod window;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::window::WindowInfo;
    pub use crate::window::msg::Message;
    pub use crate::window::msg::WindowMessage;

    pub type Result<T> = std::result::Result<T, Error>;
}
