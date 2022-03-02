// src/main.rs

#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(llvm_asm)]
#![warn(deprecated)]

use core::panic::PanicInfo;

use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
    //     }
    // }

    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    mode.draw_line((80, 60), (80, 420), Color16::White);
    mode.draw_line((80, 60), (540, 60), Color16::White);
    mode.draw_line((80, 420), (540, 420), Color16::White);
    mode.draw_line((540, 420), (540, 60), Color16::White);
    mode.draw_line((80, 90), (540, 90), Color16::White);
    for (offset, character) in "Hello World!".chars().enumerate() {
        mode.draw_character(270 + offset * 8, 72, character, Color16::White)
    }

    for (offset, character) in "Hello World!".chars().enumerate() {
        mode.draw_character(270 + offset * 8, 102, character, Color16::White)
    }

    loop {}
}
