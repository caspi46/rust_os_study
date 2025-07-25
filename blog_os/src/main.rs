
#![no_std] // disable to use std
#![no_main]

use core::panic::PanicInfo; 

mod vga_buffer; 

// This function is called on panic 
// parameter _info : PanicInfo contains: 

static HELLO: &[u8] = b"Hello World!"; 

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!"); 
    // panic(panic!("Some panic message"));
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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

// no longer find the core library:
// In core library, Result, Option, and iterators 
// Linked to all no_std crates 