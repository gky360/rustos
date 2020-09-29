pub trait PortWrite {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn write_to_port(port: u16, value: Self);
}
