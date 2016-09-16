use Uart;

pub struct Uart16550 {
    base: &'static u32
}

impl Uart16550 {
    fn new(baseaddr : u32) -> Uart16550 {
        Uart16550 { base : baseaddr }
    }
}

impl Uart for Uart16550 {
	fn put(&self, chr : u8) -> () {
        unsafe {
            let ptr : *mut usize = (self.base + 0) as *mut u32;
            *ptr = chr;
        }
    }
	fn get(&self) -> u8 {
        unsafe {
            let ptr : *const usize = (self.base + 0) as *const u32;
            *ptr
        }
    }
	fn getBaudrate(&self) -> UartBaudrate {
        B115200
    }
	fn setBaudrate(&self, b:UartBaudrate) -> () {
        ()
    }
	fn getParity(&self) -> UartParity {
        Disabled
    }

    fn setParity(&self, p:UartParity) -> () {
        ()
    }
}


