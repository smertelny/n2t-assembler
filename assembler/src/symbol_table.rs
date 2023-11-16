use std::collections::HashMap;

const SYMBOL_TABLE_CAPACITY: usize = 22;

#[derive(Debug)]
pub struct SymbolTable<'a> {
    pub table: HashMap<&'a str, usize>,
    pub symbol_index: usize,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        let mut table = HashMap::with_capacity(SYMBOL_TABLE_CAPACITY);

        table.insert("SP", 0);
        table.insert("LCL", 1);
        table.insert("ARG", 2);
        table.insert("THIS", 3);
        table.insert("THAT", 4);

        table.insert("R0", 0);
        table.insert("R1", 1);
        table.insert("R2", 2);
        table.insert("R3", 3);
        table.insert("R4", 4);
        table.insert("R5", 5);
        table.insert("R6", 6);
        table.insert("R7", 7);
        table.insert("R8", 8);
        table.insert("R9", 9);
        table.insert("R10", 10);
        table.insert("R11", 11);
        table.insert("R12", 12);
        table.insert("R13", 13);
        table.insert("R14", 14);
        table.insert("R15", 15);

        table.insert("SCREEN", 16384);
        table.insert("KBD", 24576);

        SymbolTable {
            table,
            symbol_index: 16,
        }
    }
}

pub struct InstructionTable<'a> {
    pub comp: HashMap<&'a str, u16>,
    pub dest: HashMap<&'a str, u16>,
    pub jump: HashMap<&'a str, u16>,
}

impl<'a> InstructionTable<'a> {
    pub fn new() -> Self {
        let mut dest = HashMap::with_capacity(8);
        dest.insert("null", 0b0000000000_000_000);
        dest.insert("M", 0b0000000000_001_000);
        dest.insert("D", 0b0000000000_010_000);
        dest.insert("MD", 0b0000000000_011_000);
        dest.insert("A", 0b0000000000_100_000);
        dest.insert("AM", 0b0000000000_101_000);
        dest.insert("AD", 0b0000000000_110_000);
        dest.insert("AMD", 0b0000000000_111_000);

        let mut comp = HashMap::with_capacity(28);
        // a = 0
        comp.insert("0", 0b000_0_101010_000000);
        comp.insert("1", 0b000_0_111111_000000);
        comp.insert("-1", 0b000_0_111010_000000);
        comp.insert("D", 0b000_0_001100_000000);
        comp.insert("A", 0b000_0_110000_000000);
        comp.insert("!D", 0b000_0_001101_000000);
        comp.insert("!A", 0b000_0_110001_000000);
        comp.insert("-D", 0b000_0_001101_000000);
        comp.insert("-A", 0b000_0_110011_000000);
        comp.insert("D+1", 0b000_0_011111_000000);
        comp.insert("A+1", 0b000_0_110111_000000);
        comp.insert("D-1", 0b000_0_001110_000000);
        comp.insert("A-1", 0b000_0_110010_000000);
        comp.insert("D+A", 0b000_0_000010_000000);
        comp.insert("D-A", 0b000_0_010011_000000);
        comp.insert("A-D", 0b000_0_000111_000000);
        comp.insert("D&A", 0b000_0_000000_000000);
        comp.insert("D|A", 0b000_0_010101_000000);
        // a = 1
        comp.insert("M", 0b000_1_110000_000000);
        comp.insert("!M", 0b000_1_110001_000000);
        comp.insert("-M", 0b000_1_110011_000000);
        comp.insert("M+1", 0b000_1_110111_000000);
        comp.insert("M-1", 0b000_1_110010_000000);
        comp.insert("D+M", 0b000_1_000010_000000);
        comp.insert("D-M", 0b000_1_010011_000000);
        comp.insert("M-D", 0b000_1_000111_000000);
        comp.insert("D&M", 0b000_1_000000_000000);
        comp.insert("D|M", 0b000_1_010101_000000);

        let mut jump = HashMap::with_capacity(8);
        jump.insert("null", 0b0000000000000_000);
        jump.insert("JGT", 0b0000000000000_001);
        jump.insert("JEQ", 0b0000000000000_010);
        jump.insert("JGE", 0b0000000000000_011);
        jump.insert("JLT", 0b0000000000000_100);
        jump.insert("JNE", 0b0000000000000_101);
        jump.insert("JLE", 0b0000000000000_110);
        jump.insert("JMP", 0b0000000000000_111);

        Self { comp, dest, jump }
    }
}
