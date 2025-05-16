#![no_main]
#![no_std]

use esp_println::{print, println};
use esp_hal::{main};
use vlam::{ctx, VLArray};
use esp_backtrace::*;

#[inline(never)]
fn foo(n: usize){
    ctx!(array_context);

    let mut buffer: VLArray<u8> = array_context.zeroed_buffer(n);
    let other_buffer: VLArray<u32> = array_context.array_from_exact_size_iterator(0..255);

    for i  in 0..n {
        buffer[i] = (i & 0xFF) as u8;
    }

    for item in &mut buffer{
        *item *= 2;
    }

    print!("[");
    for item in &buffer {
        print!("{}, ", item);
    }
    println!("]");

    print!("[");
    for item in &other_buffer {
        print!("{}, ", item);
    }
    println!("]");
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Hello, world!");

    foo(16);

    println!("Hello, again!");

    loop {
        // println!("tick!")
    }
}
