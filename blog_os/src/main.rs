
#![no_std] // disable to use std
#![no_main]

use core::panic::PanicInfo; 

// This function is called on panic 
// parameter _info : PanicInfo contains: 
// the file and line where the panic happend and the optional panic message
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// A Freestanding Rust Binary by Philipp Oppermann's blog 

// disable the standard library 
// since  they depends on the operating system features like threads, files, or networking 
// and includes C standard library libc 
// => can't use them (OS-Dependant libraries) due to creating our own operating system 


// start attribute: 
// main function = the first function called when you run the program 
// need to initialize itself before calling main => runtime system
// runtime system for garbage collection or thread for programming languages 

// Linker error
// Linker : a program that combines the generated code into an executable 

// Command line: compiler for the host system with the linker argument
// cargo rustc -- -C link-args="-e __start -static -nostartfiles"