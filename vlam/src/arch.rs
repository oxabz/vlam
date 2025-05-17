
#[cfg(not(any(target_arch = "riscv32")))]
compile_error!("Not  implemented for this architecture");



#[cfg(target_arch = "riscv32")]
pub(crate) mod riscv32;
#[cfg(target_arch = "riscv32")]
pub(crate) use riscv32::*;