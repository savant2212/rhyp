#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]

// kernel.rs
#[no_mangle]
pub extern fn main() {  
	loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}  