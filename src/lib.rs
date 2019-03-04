#![allow(unused_attributes)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod gdt;
mod interrupts;
mod serial;
mod vga;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use interrupts::PICS;

    println!("Hi!");
    vga::ferris_say("This is TetanOS");

    interrupts::init();
    gdt::init();

    unsafe { PICS.lock().initialize() };

    println!("Be careful, it's kinda rusty in here.");

    x86_64::instructions::interrupts::enable();

    loop {
        print!("-");
        for _ in 0..10000 {}
    }
}
