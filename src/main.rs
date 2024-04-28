#![no_std]
#![no_main]                         // We need to specify we don't want an entry point into a runtime.

use core::panic::PanicInfo;

#![no_mangle]                       // This ensure that the function name is "_start" and no other
pub extern "C" fn _start() -> ! {   // This is important since we need to tell the linker our entry point.
    loop{}                          // We extern "C" to use C naming convetions as it's usually the naming 
                                    // convetions for most systems.
}

#[panic_handler]                    // Attribute that defines the fn that will be invoked
fn panic(_info: &PanicInfo) -> ! {  // when a panic occurs.  ( Panic == Unrercoverable error )
    loop {}                         // This is a diverging Function. It never returns.
}
