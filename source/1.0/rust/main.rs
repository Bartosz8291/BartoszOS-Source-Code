#![no_std]
#![no_main]

use core::panic::PanicInfo;

// VGA text mode constants
const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
const VGA_WIDTH: usize = 80;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point of the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_str("Welcome to BartoszOS");
    initialize_os();

    // Main loop of the OS
    loop {
        // Your OS logic goes here
    }
}

// Function to initialize the OS
fn initialize_os() {
    // Initialization code goes here
    // Example: setting up memory, initializing hardware, etc.
}

// Function to print a string to VGA text mode
fn print_str(s: &str) {
    let mut buffer_ptr = VGA_BUFFER;

    for byte in s.bytes() {
        unsafe {
            *buffer_ptr = byte as u16 | 0x0f00;
            buffer_ptr = buffer_ptr.offset(1);
        }
    }
}
