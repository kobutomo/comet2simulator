/// 命令の数
const NUM: usize = 38;
const OPCODE: [i16; NUM] = [
    0x00, 0x10, 0x11, 0x12, 0x14, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x30, 0x31, 0x32,
    0x34, 0x35, 0x36, 0x40, 0x41, 0x44, 0x45, 0x50, 0x51, 0x52, 0x53, 0x61, 0x62, 0x63, 0x64, 0x65,
    0x66, 0x70, 0x71, 0x80, 0x81, 0xf0,
];
const SIZE: [i16; NUM] = [
    1, 2, 2, 2, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 1, 2, 1, 2,
];
const MNEMONIC: [&'static str; NUM] = [
    "NOP", "LD", "ST", "LAD", "LD", "ADDA", "SUBA", "ADDL", "SUBL", "ADDA", "SUBA", "ADDL", "SUBL",
    "AND", "OR", "XOR", "AND", "OR", "XOR", "CPA", "CPL", "CPA", "CPL", "SLA", "SRA", "SLL", "SRL",
    "JMI", "JNZ", "JZE", "JUMP", "JPL", "JOV", "PUSH", "POP", "CALL", "RET", "SVC",
];

#[derive(Debug, PartialEq)]
struct Instructions {
    pr: i16,
    gr: [i16; 8],
    ir: [i16; 2],
    mr: i16,
    mdr: i16,
    eadr: i16,
    fr: [bool; 3],
    sp: i16,
}

impl Instructions {
    pub fn get_size(op: i16) -> i16 {
        for i in 0..NUM {
            if OPCODE[i] == op {
                SIZE[i];
            }
        }
        0
    }
    pub fn get_mnemonic(op: i16) -> &'static str {
        for i in 0..NUM {
            if OPCODE[i] == op {
                MNEMONIC[i];
            }
        }
        ""
    }
}
