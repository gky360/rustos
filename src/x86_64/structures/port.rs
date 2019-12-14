pub trait PortWrite {
    unsafe fn write_to_port(port: u16, value: Self);
}
