#[cfg(feature = "color")]
pub mod clog;
#[cfg(feature = "color")]
pub use clog::clog;
pub mod flog;
pub mod levels;
pub mod plog;
pub use flog::flog;
pub use plog::log;
