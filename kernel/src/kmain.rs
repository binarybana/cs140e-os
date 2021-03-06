#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]

#![feature(pointer_methods)]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

// #[no_mangle]
// pub extern "C" fn kmain() {
//     use std::io::Write;
//
//     let mut m = pi::uart::MiniUart::new();
//     loop {
//         let b = m.read_byte();
//         m.write_byte(b);
//         write!(m, "<-");
//     }
// }

pub fn blink(times: u8, interval: u64) {
    use pi::timer::spin_sleep_ms;

    let mut gpio16 = pi::gpio::Gpio::new(16).into_output();
    for _ in 0..times {
       gpio16.set();
       spin_sleep_ms(interval);
       gpio16.clear();
       spin_sleep_ms(interval);
    }
}

pub fn echo() {
    use pi::timer::spin_sleep_ms;
    use pi::uart::MiniUart;
    use std::fmt::Write;

    let mut uart = MiniUart::new();

    loop {
        uart.write_str("\n<-");
        let byte = uart.read_byte();
        uart.write_byte(byte);
    }
}

pub fn shell() {
    use console::{kprint, kprintln};
    use std::fmt::Write;
    use pi::timer::spin_sleep_ms;
    loop {
        kprintln!("Hello world");
        kprint!("Hello to ");
        kprint!("you too!");
        spin_sleep_ms(200);
    }
    // let mut console = console::CONSOLE.lock();
    // loop {
    //     console.write_str("\n<-");
    //     let b = console.read_byte();
    //     console.write_byte(b);
    //     // kprintln!("$ {}", b);
    // }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // blink(3, 1000);
    // echo();
    // shell();
    shell::shell("> ");
}
