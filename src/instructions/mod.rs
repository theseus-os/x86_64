//! Low level functions for special x86 instructions.

pub mod port;
pub mod interrupts;
pub mod tables;
pub mod tlb;
pub mod segmentation;

use core::arch::asm;

/// Halts the CPU by executing the `hlt` instruction.
#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt", options(nomem, nostack, preserves_flags));
}

// Model specific registers

/// Write 64 bits to msr register.
pub unsafe fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low, in("edx") high,
        options(nostack, preserves_flags),
    );
}

/// Read 64 bits msr register.
pub fn rdmsr(msr: u32) -> u64 {
    let (high, low): (u32, u32);
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low, out("edx") high,
            options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}

/// Read 64 bit PMC (performance monitor counter).
pub fn rdpmc(msr: u32) -> u64 {
    let (high, low): (u32, u32);
    unsafe {
        asm!(
            "rdpmc",
            in("ecx") msr,
            out("eax") low, out("edx") high,
            options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}
