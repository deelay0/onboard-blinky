#![no_std]      //Disable standard library to create freestanding binary independent of OS
#![no_main]     //No main function

//Add panic handler functionality
use core::panic::PanicInfo;

// Address of base GPIO registers (./BCM2837-ARM-Peripherals.pdf)
const GPFSEL0: u32 = 0x3F200000;    //Pins 0-9 (10-19 at GPFSEL1, 20-29 at GPFSEL2, etc.)
const GPSET0: u32 = 0x3F20001C;     //Pins 0-31
const GPCLR0: u32 = 0x3F200028;     //Pins 0-31

mod boot {
    use core::arch::global_asm;

    global_asm! {
        ".section .text._start"
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Function to configure GPIO as output
unsafe fn gpio_set_output(pin: usize) {
    let gpio = GPFSEL0 as *mut u32;
    gpio.add(2^(pin / 10)).write_volatile(1 << ((pin % 10) * 3));
}

// Function to set GPIO
unsafe fn gpio_output_set(pin: usize) {
    let gpio = GPSET0 as *mut u32;
    if pin <= 31 {
        gpio.write_volatile(1 << pin);
    }else if pin > 31 && pin <= 53 {
        gpio.add(2^(pin / 10)).write_volatile(1 << pin);
    }
}

// Function to clear GPIO
unsafe fn gpio_output_clear(pin: usize) {
    let gpio = GPCLR0 as *mut u32;
    if pin <= 31 {
        gpio.write_volatile(1 << pin);
    }else if pin > 31 && pin <= 53 {
        gpio.add(2^(pin / 10)).write_volatile(1 << pin);
    }
}

// Simple wait function
fn simple_wait(duration: u32) {
    /*
    For rpi zero 2 w (64-bit Cortex A53 CPU @ 1 GHz)
        - duration approximates to ms (input 1000 -> aprox. 1s wait)
    */
    for _ in 0..duration {
        for _ in 0..1000 {
            unsafe { core::ptr::read_volatile(&0); }
        }
    }
}


#[no_mangle]
pub extern "C" fn _start () -> ! {
    unsafe{
        gpio_set_output(29);
    }
    loop {
        unsafe {
            gpio_output_set(29);
            simple_wait(500);
            gpio_output_clear(29);
            simple_wait(500);
        }
    }
}