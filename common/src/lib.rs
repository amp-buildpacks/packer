extern crate self as packer_common;

pub mod errors;
pub mod fs;
pub mod git;
pub use git::Git;

/// Conditionally print a message
///
/// This macro accepts a predicate and the message to print if the predicate is tru
///
/// ```ignore
/// let quiet = true;
/// p_println!(!quiet => "message");
/// ```
#[macro_export]
macro_rules! p_println {
    ($p:expr => $($arg:tt)*) => {{
        if $p {
            println!($($arg)*)
        }
    }}
}
