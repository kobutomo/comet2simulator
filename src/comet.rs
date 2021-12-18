use crate::instructions;
use crate::memory;

const OF: usize = 0;
const SF: usize = 1;
const ZF: usize = 2;
#[derive(Debug, PartialEq)]
pub struct Comet {
    pr: u16,
    gr: [u16; 8],
    ir: [u16; 2],
    mar: u16,
    mdr: u16,
    eadr: u16,
    fr: [bool; 3],
    sp: u16,
    executed_address: [u16; 2],
    end_flag: bool,
    pub main_memory: memory::Memory,
}

impl Comet {
    pub fn new() -> Self {
        Self {
            pr: 0xFFFFu16,
            gr: [0xFFFFu16; 8],
            ir: [0xFFFFu16; 2],
            mar: 0xFFFFu16,
            mdr: 0xFFFFu16,
            eadr: 0xFFFFu16,
            fr: [false; 3],
            sp: 0xFFFFu16,
            executed_address: [0xFFFFu16; 2],
            end_flag: false,
            main_memory: memory::new(),
        }
    }

    pub fn get_pr(&self) -> &u16 {
        &self.pr
    }
    pub fn get_ir(&self, i: usize) -> &u16 {
        self.ir.get(i).unwrap()
    }
    pub fn get_gr(&self, i: usize) -> &u16 {
        self.gr.get(i).unwrap()
    }
    pub fn get_mar(&self) -> &u16 {
        &self.mar
    }
    pub fn get_mdr(&self) -> &u16 {
        &self.mdr
    }
    pub fn get_sp(&self) -> &u16 {
        &self.sp
    }
    pub fn get_eadr(&self) -> &u16 {
        &self.eadr
    }
    pub fn get_fr(&self, i: usize) -> &bool {
        &self.fr[i]
    }

    pub fn get_current_operation(&self) -> Option<&str> {
        let op = {
            if self.executed_address[0] != 0xFFFFu16 {
                let raw_op = *self.main_memory.read(self.executed_address[0]).unwrap();
                let op_code = raw_op >> 8 & 0x00FF;
                Some(instructions::get_mnemonic(op_code))
            } else {
                None
            }
        };
        op
    }

    fn set_flag(&mut self, data: u16) {
        self.fr[OF] = false;
        if data == 0 {
            self.fr[SF] = false;
            self.fr[ZF] = true;
        } else if data > 0 {
            self.fr[SF] = false;
            self.fr[ZF] = false;
        } else {
            self.fr[SF] = true;
            self.fr[ZF] = false;
        }
    }

    pub fn init(&mut self, raw_code: String) {
        raw_code.split("\n").enumerate().for_each(|(i, s)| {
            if i == 0 {
                self.pr = 0;
            }
            let without_prefix = s.trim_start_matches("0x");
            let op = u16::from_str_radix(without_prefix, 16).unwrap();
            self.main_memory.write(i as u16, op);
        })
    }

    pub fn execute_step(&mut self) {
        self.executed_address[0] = 0xFFFFu16;
        self.executed_address[1] = 0xFFFFu16;
        if self.pr == 0xFFFFu16 || self.end_flag {
            return;
        }
        // 命令取り出し
        let (op_code, length) = self.read_operation();
        // 命令解読
        let (gr1, gr2) = self.decode_operation();
        // アドレス生成

        if length == 2 {
            self.eadr = self.generate_address(gr2);
        }
        self.execute(op_code, gr1, gr2);
    }

    /// 命令取り出しサイクル
    fn read_operation(&mut self) -> (u16, u16) {
        // 一語目を取り出す
        self.executed_address[0] = self.pr;
        self.mar = self.pr;
        self.pr += 1;
        self.mdr = *self.main_memory.read(self.mar).unwrap();
        self.ir[0] = self.mdr;
        // 命令コード取得 16bit の命令を右に 8bit シフトして下位 8bit をマスクする．
        let op_code = self.ir[0] >> 8 & 0x00FF;
        let length = instructions::get_size(op_code);
        if length == 2 {
            self.executed_address[1] = self.pr;
            self.mar = self.pr;
            self.pr += 1;
            self.mdr = *self.main_memory.read(self.mar).unwrap();
            self.ir[1] = self.mdr;
        };
        (op_code, length)
    }

    /// 命令解読サイクル
    fn decode_operation(&self) -> (u16, u16) {
        // オペランド取得
        let gr1 = self.ir[0] >> 4 & 0x000F;
        let gr2 = self.ir[0] & 0x000F;
        return (gr1, gr2);
    }

    /// アドレス生成サイクル
    fn generate_address(&self, gr2: u16) -> u16 {
        if gr2 == 0 {
            self.ir[1]
        } else {
            gr2 + self.ir[1]
        }
    }

    /// 命令実行サイクル
    fn execute(&mut self, op_code: u16, gr1: u16, _gr2: u16) {
        match op_code {
            0x00 => (),
            // LD
            0x10 => {
                self.mar = self.eadr;
                self.mdr = *self.main_memory.read(self.mar).unwrap();
                self.gr[gr1 as usize] = self.mdr;
                self.set_flag(self.gr[gr1 as usize]);
            }
            // ST
            0x11 => {
                self.mar = self.eadr;
                self.mdr = self.gr[gr1 as usize];
                self.main_memory.write(self.mar, self.mdr);
            }
            // ADDA 2
            0x20 => {
                self.mar = self.eadr;
                self.mdr = *self.main_memory.read(self.mar).unwrap();
                let result = self.gr[gr1 as usize] + self.mdr;
                self.set_flag(result);
                self.gr[gr1 as usize] = result;
            }
            // RET
            0x81 => {
                self.end_flag = true;
            }
            _ => (),
        }
    }
}
