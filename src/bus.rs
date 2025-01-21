pub trait Bus {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn read_word(&mut self, addr: u16) -> u16;
    fn write_byte(&mut self, addr: u16, data: u8);
    fn write_word(&mut self, addr: u16, data: u16);
    fn read_io_byte(&mut self, port: u8) -> u8;
    fn write_io_byte(&mut self, port: u8, data: u8);
}
