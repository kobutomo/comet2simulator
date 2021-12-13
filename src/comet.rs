use crate::memory;

#[derive(Debug, PartialEq)]
pub struct Comet {
    pr: i16,
    gr: [i16; 8],
    ir: [i16; 2],
    mar: i16,
    mdr: i16,
    eadr: i16,
    fr: [bool; 3],
    sp: i16,
    pub main_memory: memory::Memory,
}

impl Comet {
    const OF: usize = 0;
    const SF: usize = 1;
    const ZF: usize = 2;

    pub fn new() -> Self {
        Self {
            pr: -1i16,
            gr: [-1i16; 8],
            ir: [-1i16; 2],
            mar: -1i16,
            mdr: -1i16,
            eadr: -1i16,
            fr: [false; 3],
            sp: -1i16,
            main_memory: memory::new(),
        }
    }

    pub fn get_pr(&self) -> &i16 {
        &self.pr
    }
    pub fn get_ir(&self, i: usize) -> &i16 {
        self.ir.get(i).unwrap()
    }
    pub fn get_gr(&self, i: usize) -> &i16 {
        self.gr.get(i).unwrap()
    }
    pub fn get_mar(&self) -> &i16 {
        &self.mar
    }
    pub fn get_mdr(&self) -> &i16 {
        &self.mdr
    }
    pub fn get_sp(&self) -> &i16 {
        &self.sp
    }
    pub fn get_eadr(&self) -> &i16 {
        &self.eadr
    }
    pub fn get_fr(&self, i: usize) -> &bool {
        &self.fr[i]
    }
}
