#![no_std]


pub mod x86;
pub mod x86_64;


#[cfg(target_arch = "x86")]
pub use x86::{IdtDescriptor, IdtEntry};

#[cfg(target_arch = "x86_64")]
pub use x86_64::{IdtDescriptor, IdtEntry};