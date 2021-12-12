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
        let mm = self.mm.as_mut();
        let ptr = mm.get_mut(mar as usize);
        match ptr {
            Some(ptr) => *ptr = mdr,
            None => (),
        };
    }
}

pub fn new() -> Memory {
    Memory { mm: [0; 64] }
}
