use crate::x86_64::instructions::port::Port;

const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;
const MODE_8086: u8 = 0x01;

struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

impl Pic {
    const fn new(offset: u8, command: Port<u8>, data: Port<u8>) -> Pic {
        Pic {
            offset,
            command,
            data,
        }
    }

    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }

    unsafe fn enable_interrupt(&mut self, interrupt_id: u8) {
        let interrupt_mask = 0b1111_1111 ^ (1 << (interrupt_id - self.offset));
        self.data.write(interrupt_mask);
    }
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic::new(offset1, Port::new(0x20), Port::new(0x21)),
                Pic::new(offset2, Port::new(0xA0), Port::new(0xA1)),
            ],
        }
    }

    pub unsafe fn initialize(&mut self) {
        for (i, pic) in self.pics.iter_mut().enumerate() {
            let interrupt_mask = 0b1111_1111;
            pic.data.write(interrupt_mask);

            pic.command.write(CMD_INIT);
            pic.data.write(pic.offset);
            pic.data.write(if i == 0 { 0b0000_0100 } else { 2 });
            pic.data.write(MODE_8086);

            let interrupt_mask = if i == 0 { 0b1111_1011 } else { 0b1111_1111 };
            pic.data.write(interrupt_mask);
        }
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        for pic in self.pics.iter_mut() {
            if pic.handles_interrupt(interrupt_id) {
                pic.end_of_interrupt();
            }
        }
    }

    pub unsafe fn enable_interrupt(&mut self, interrupt_id: u8) {
        for pic in self.pics.iter_mut() {
            if pic.handles_interrupt(interrupt_id) {
                pic.enable_interrupt(interrupt_id);
            }
        }
    }
}
