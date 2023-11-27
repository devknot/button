
use riscv::{asm, interrupt};
use core::panic;

#[panic_handler]
fn panic(_: &panic::PanicInfo) -> ! {
    unsafe {
        interrupt::disable();
        
        loop {
            asm::ebreak();
        }
    }
}

