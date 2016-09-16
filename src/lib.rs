#![feature(lang_items, asm, start)]
#![crate_type = "staticlib"]
#![no_std]

extern crate rlibc;

#[no_mangle]
#[cfg(any(target_arch = "arm", target_arch = "armv7"))]
pub extern fn _entry() {
    unsafe {
        asm!("
			b main
		");
    }
		panic!()
}

// kernel.rs
#[no_mangle]
pub extern fn main() {  
	let mut l = 0;	
	while l < 1024 {
		l += 1;
	}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}  

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
        loop {}
}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
}
