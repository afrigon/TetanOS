use super::interrupt;
use super::segmentation;
use crate::kernel_main;

/// Entry point of the kernel for the x86_64 architecture.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    segmentation::global_descriptor_table::init();
    interrupt::descriptor_table::init();

    interrupt::breakpoint();
    kernel_main();
}
