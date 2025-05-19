# VLAM !!!

This crate is an implementation of stack allocated variable length array.
For now this crate only support riscv32 baremetal targets.

## How does it works ?

This crate works by pushing the stack pointer using inline ASM instructions and restoring it before leaving the function.
This is done through the context struct. Which save the stack frame at it's creation and restore it when it's dropped.

## Usage

```rust

#[vlam::vlam(ctx)]
fn foo(n: usize) {
    let zeroed: vlam::VLArray<u8> = ctx.zeroed_buffer(n);
    let iter_init: vlam::VLArray<u32> = ctx.array_from_exact_size_iterator(0..255);
    let from_slice: vlam::VLArray<u32> = ctx.array_from_exact_size_iterator(&iter_init[0..20]);
}

```