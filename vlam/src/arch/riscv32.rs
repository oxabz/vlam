macro_rules! save_stack_pointer {
    ($stack_pointer:ident) => {
        core::arch::asm!(
            "mv {stack_pointer}, sp",
            stack_pointer = out(reg) $stack_pointer,
        )
    };
}

macro_rules! restore_stack_pointer {
    ($stack_pointer:ident) => {
        core::arch::asm!(
            "mv sp, {stack_pointer}",
            stack_pointer = in(reg) $stack_pointer,
        )
    };
}

macro_rules! allocate_on_stack {
    ($origin:ident, $size:ident) => {
        core::arch::asm!(
        "mv {origin}, sp",
        "sub sp, sp, {size}",
        origin = out(reg) $origin,
        size = in(reg) $size
        );
    };
}

pub(crate) const STACK_ALIGNMENT: usize = 16;

pub(crate) use {save_stack_pointer, restore_stack_pointer, allocate_on_stack};