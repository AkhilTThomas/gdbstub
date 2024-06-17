use core::num::NonZeroUsize;
use gdbstub::arch::RegId;

/// 32-bit TriCore core register identifier.
#[derive(Debug, Clone, Copy)]
#[repr(usize)] // Ensure the enum is represented as a usize for conversion.
#[non_exhaustive]
pub enum TriCoreCoreRegId {
    /// General purpose Data register D0
    D0,
    /// General purpose Data register D1
    D1,
    /// General purpose Data register D2
    D2,
    /// General purpose Data register D3
    D3,
    /// General purpose Data register D4
    D4,
    /// General purpose Data register D5
    D5,
    /// General purpose Data register D6
    D6,
    /// General purpose Data register D7
    D7,
    /// General purpose Data register D8
    D8,
    /// General purpose Data register D9
    D9,
    /// General purpose Data register D10
    D10,
    /// General purpose Data register D11
    D11,
    /// General purpose Data register D12
    D12,
    /// General purpose Data register D13
    D13,
    /// General purpose Data register D14
    D14,
    /// General purpose Data register D15
    D15,
    /// General purpose Address register A0
    A0,
    /// General purpose Address register A1
    A1,
    /// General purpose Address register A2
    A2,
    /// General purpose Address register A3
    A3,
    /// General purpose Address register A4
    A4,
    /// General purpose Address register A5
    A5,
    /// General purpose Address register A6
    A6,
    /// General purpose Address register A7
    A7,
    /// General purpose Address register A8
    A8,
    /// General purpose Address register A9
    A9,
    /// General purpose Address register A10
    A10,
    /// General purpose Address register A11
    A11,
    /// General purpose Address register A12
    A12,
    /// General purpose Address register A13
    A13,
    /// General purpose Address register A14
    A14,
    /// General purpose Address register A15
    A15,
    /// LCX register
    Lcx,
    /// FCX register
    Fcx,
    /// Previous Context Information register
    Pcxi,
    /// Processor Status Word
    Psw,
    /// Program Counter
    Pc,
    /// Interrupt Control Register
    Icr,
    /// Interrupt Stack Pointer
    Isp,
    /// Base Trap Vector
    Btv,
    /// Base Interrupt Vector
    Biv,
    /// System Configuration Register
    Syscon,
    /// Performance Monitor Control Register 0
    Pmucon0,
    /// Data Memory Unit Control Register
    Dmucon,
}

impl RegId for TriCoreCoreRegId {
    fn from_raw_id(id: usize) -> Option<(Self, Option<NonZeroUsize>)> {
        let reg = match id {
            0 => TriCoreCoreRegId::D0,
            1 => TriCoreCoreRegId::D1,
            2 => TriCoreCoreRegId::D2,
            3 => TriCoreCoreRegId::D3,
            4 => TriCoreCoreRegId::D4,
            5 => TriCoreCoreRegId::D5,
            6 => TriCoreCoreRegId::D6,
            7 => TriCoreCoreRegId::D7,
            8 => TriCoreCoreRegId::D8,
            9 => TriCoreCoreRegId::D9,
            10 => TriCoreCoreRegId::D10,
            11 => TriCoreCoreRegId::D11,
            12 => TriCoreCoreRegId::D12,
            13 => TriCoreCoreRegId::D13,
            14 => TriCoreCoreRegId::D14,
            15 => TriCoreCoreRegId::D15,
            16 => TriCoreCoreRegId::A0,
            17 => TriCoreCoreRegId::A1,
            18 => TriCoreCoreRegId::A2,
            19 => TriCoreCoreRegId::A3,
            20 => TriCoreCoreRegId::A4,
            21 => TriCoreCoreRegId::A5,
            22 => TriCoreCoreRegId::A6,
            23 => TriCoreCoreRegId::A7,
            24 => TriCoreCoreRegId::A8,
            25 => TriCoreCoreRegId::A9,
            26 => TriCoreCoreRegId::A10,
            27 => TriCoreCoreRegId::A11,
            28 => TriCoreCoreRegId::A12,
            29 => TriCoreCoreRegId::A13,
            30 => TriCoreCoreRegId::A14,
            31 => TriCoreCoreRegId::A15,
            32 => TriCoreCoreRegId::Lcx,
            33 => TriCoreCoreRegId::Fcx,
            34 => TriCoreCoreRegId::Pcxi,
            35 => TriCoreCoreRegId::Psw,
            36 => TriCoreCoreRegId::Pc,
            37 => TriCoreCoreRegId::Icr,
            38 => TriCoreCoreRegId::Isp,
            39 => TriCoreCoreRegId::Btv,
            40 => TriCoreCoreRegId::Biv,
            41 => TriCoreCoreRegId::Syscon,
            42 => TriCoreCoreRegId::Pmucon0,
            43 => TriCoreCoreRegId::Dmucon,
            _ => return None,
        };
        Some((reg, NonZeroUsize::new(4)))
    }
}
