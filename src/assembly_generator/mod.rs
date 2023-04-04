#[cfg(target_arch = "x86_64")] pub mod x86_64;
#[cfg(target_arch = "arm")] pub mod arm;

#[cfg(target_arch = "x86_64")] pub use self::x86_64 as current;
#[cfg(target_arch = "arm")] pub use self::arm as current;

pub mod error;
