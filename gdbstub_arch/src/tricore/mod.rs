//! Implementation for the tricore 1.6V architecture

use gdbstub::arch::Arch;

pub mod reg;

/// Implements `Arch` for Tricore V1.6
pub enum TricoreV1_6 {}

impl Arch for TricoreV1_6 {
    type Usize = u32;
    type Registers = reg::TricoreCoreRegs;
    type RegId = reg::id::TriCoreCoreRegId;
    type BreakpointKind = usize;

    //todo: V1_6 string format seems to break gdb, is _ not allowed?
    // refer: https://sourceware.org/gdb/current/onlinedocs/gdb.html/Target-Description-Format.html

    // fn target_description_xml() -> Option<&'static str> {
    //     Some(r#"<target version="1.0"><architecture>TriCore:V1_6</architecture></target>"#)
    // }
}
