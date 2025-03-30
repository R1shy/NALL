pub mod plog;
pub mod flog;
pub mod levels;
#[cfg(feature = "color")]
pub mod clog;
pub mod config;
pub use plog::log;
pub use flog::flog;

pub mod constants {
 pub static mut USE_COLOR: bool = false;
}
