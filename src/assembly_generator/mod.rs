#[cfg(target_arch = "x86_64")] pub mod x86_64;
#[cfg(target_arch = "arm")] pub mod arm;

#[cfg(target_arch = "x86_64")] use self::x86_64 as current;
#[cfg(target_arch = "arm")] use self::arm as current;

pub mod error;
