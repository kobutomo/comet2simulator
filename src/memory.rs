#[derive(Debug, PartialEq)]
pub struct Memory {
    mm: [i16; Memory::SIZE as usize],
}

impl Memory {
    const SIZE: i16 = 64;

    pub fn read(&self, mar: i16) -> Option<&i16> {
        self.mm.get(mar as usize)
    }
    pub fn write(&mut self, mar: i16, mdr: i16) {
        let mm = &mut self.mm;
        let op = mm.get_mut(mar as usize);
        let ptr = op.unwrap();
        *ptr = mdr;
    }
}

pub fn new() -> Memory {
    Memory { mm: [-1i16; 64] }
}
