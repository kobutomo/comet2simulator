#[derive(Debug, PartialEq)]
pub struct Memory {
    mm: [u16; Memory::SIZE as usize],
}

impl Memory {
    const SIZE: u16 = 64;

    pub fn read(&self, mar: u16) -> Option<&u16> {
        self.mm.get(mar as usize)
    }
    pub fn write(&mut self, mar: u16, mdr: u16) {
        let mm = &mut self.mm;
        let op = mm.get_mut(mar as usize);
        let ptr = op.unwrap();
        *ptr = mdr;
    }
}

pub fn new() -> Memory {
    Memory { mm: [0xFFFFu16; 64] }
}
