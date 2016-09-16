enum UartBaudrate {
	B110, 
	B150, 
	B300, 
	B1200, 
	B2400, 
	B4800, 
	B9600, 
	B19200, 
	B38400, 
	B57600, 
	B115200, 
	B230400, 
	B460800, 
	B921600
}

enum UartParity {
	Odd,
	Even,
	Disabled
}

enum UartStopbits {
	Zero,
	One,
	Two
}

trait Uart {
	fn put(&self, chr : u8) -> ();
	fn get(&self) -> u8;
	fn getBaudrate(&self) -> UartBaudrate;
	fn setBaudrate(&self, b: UartBaudrate) -> ();
	fn getParity(&self) -> UartParity;
    fn setParity(&self, p: UartParity ) -> ();
}
