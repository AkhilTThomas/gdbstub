use core::convert::TryInto;
use gdbstub::arch::Registers;
/// 32-bit Tricore core registers.
///
/// Source: <tricore-gdb\src\gdb-11.2\gdb\features\tricore-core.xml>
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TricoreCoreRegs {
    /// General purpose Data register D0
    pub d0: u32,
    /// General purpose Data register D1
    pub d1: u32,
    /// General purpose Data register D2
    pub d2: u32,
    /// General purpose Data register D3
    pub d3: u32,
    /// General purpose Data register D4
    pub d4: u32,
    /// General purpose Data register D5
    pub d5: u32,
    /// General purpose Data register D6
    pub d6: u32,
    /// General purpose Data register D7
    pub d7: u32,
    /// General purpose Data register D8
    pub d8: u32,
    /// General purpose Data register D9
    pub d9: u32,
    /// General purpose Data register D10
    pub d10: u32,
    /// General purpose Data register D11
    pub d11: u32,
    /// General purpose Data register D12
    pub d12: u32,
    /// General purpose Data register D13
    pub d13: u32,
    /// General purpose Data register D14
    pub d14: u32,
    /// General purpose Data register D15
    pub d15: u32,
    /// General purpose Address register A0
    pub a0: u32,
    /// General purpose Address register A1
    pub a1: u32,
    /// General purpose Address register A2
    pub a2: u32,
    /// General purpose Address register A3
    pub a3: u32,
    /// General purpose Address register A4
    pub a4: u32,
    /// General purpose Address register A5
    pub a5: u32,
    /// General purpose Address register A6
    pub a6: u32,
    /// General purpose Address register A7
    pub a7: u32,
    /// General purpose Address register A8
    pub a8: u32,
    /// General purpose Address register A9
    pub a9: u32,
    /// General purpose Address register A10
    pub a10: u32,
    /// General purpose Address register A11
    pub a11: u32,
    /// General purpose Address register A12
    pub a12: u32,
    /// General purpose Address register A13
    pub a13: u32,
    /// General purpose Address register A14
    pub a14: u32,
    /// General purpose Address register A15
    pub a15: u32,
    /// LCX register
    pub lcx: u32,
    /// FCX register
    pub fcx: u32,
    /// Previous Context information
    pub pcxi: u32,
    /// Processor Status word
    pub psw: u32,
    /// Program counter
    pub pc: u32,
    /// Interrupt Control Register
    pub icr: u32,
    /// Interrupt Stack Pointer
    pub isp: u32,
    /// Base Trap Vector
    pub btv: u32,
    /// Base Interrupt Vector
    pub biv: u32,
    /// System Configuration Register
    pub syscon: u32,
    /// Performance Monitor Control Register 0
    pub pmucon0: u32,
    /// Data Memory Unit Control Register
    pub dmucon: u32,
}

impl Registers for TricoreCoreRegs {
    type ProgramCounter = u32;

    fn pc(&self) -> Self::ProgramCounter {
        self.pc
    }

    fn gdb_serialize(&self, mut write_byte: impl FnMut(Option<u8>)) {
        // Create a slice of all registers
        let regs: &[u32] = &[
            self.d0,
            self.d1,
            self.d2,
            self.d3,
            self.d4,
            self.d5,
            self.d6,
            self.d7,
            self.d8,
            self.d9,
            self.d10,
            self.d11,
            self.d12,
            self.d13,
            self.d14,
            self.d15,
            self.a0,
            self.a1,
            self.a2,
            self.a3,
            self.a4,
            self.a5,
            self.a6,
            self.a7,
            self.a8,
            self.a9,
            self.a10,
            self.a11,
            self.a12,
            self.a13,
            self.a14,
            self.a15,
            self.lcx,
            self.fcx,
            self.pcxi,
            self.psw,
            self.pc,
            self.icr,
            self.isp,
            self.btv,
            self.biv,
            self.syscon,
            self.pmucon0,
            self.dmucon,
        ];

        // Serialize each register as little-endian bytes
        for &reg in regs {
            for &byte in &reg.to_le_bytes() {
                write_byte(Some(byte));
            }
        }
    }

    fn gdb_deserialize(&mut self, bytes: &[u8]) -> Result<(), ()> {
        if bytes.len() != 44 * 4 {
            return Err(());
        }

        let mut bytes = bytes.chunks_exact(4);

        // Helper macro to read the next u32 from the byte chunks
        macro_rules! next_u32 {
            () => {
                u32::from_le_bytes(bytes.next().ok_or(())?.try_into().map_err(|_| ())?)
            };
        }

        // Deserialize each register in the same order
        self.d0 = next_u32!();
        self.d1 = next_u32!();
        self.d2 = next_u32!();
        self.d3 = next_u32!();
        self.d4 = next_u32!();
        self.d5 = next_u32!();
        self.d6 = next_u32!();
        self.d7 = next_u32!();
        self.d8 = next_u32!();
        self.d9 = next_u32!();
        self.d10 = next_u32!();
        self.d11 = next_u32!();
        self.d12 = next_u32!();
        self.d13 = next_u32!();
        self.d14 = next_u32!();
        self.d15 = next_u32!();
        self.a0 = next_u32!();
        self.a1 = next_u32!();
        self.a2 = next_u32!();
        self.a3 = next_u32!();
        self.a4 = next_u32!();
        self.a5 = next_u32!();
        self.a6 = next_u32!();
        self.a7 = next_u32!();
        self.a8 = next_u32!();
        self.a9 = next_u32!();
        self.a10 = next_u32!();
        self.a11 = next_u32!();
        self.a12 = next_u32!();
        self.a13 = next_u32!();
        self.a14 = next_u32!();
        self.a15 = next_u32!();
        self.lcx = next_u32!();
        self.fcx = next_u32!();
        self.pcxi = next_u32!();
        self.psw = next_u32!();
        self.pc = next_u32!();
        self.icr = next_u32!();
        self.isp = next_u32!();
        self.btv = next_u32!();
        self.biv = next_u32!();
        self.syscon = next_u32!();
        self.pmucon0 = next_u32!();
        self.dmucon = next_u32!();

        Ok(())
    }
}
