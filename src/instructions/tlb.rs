//! Functions to flush the translation lookaside buffer (TLB).

use VirtualAddress;
use core::arch::asm;

/// Invalidate the given address in the TLB using the `invlpg` instruction.
pub fn flush(addr: VirtualAddress) {
    unsafe {
        asm!("invlpg [{}]", in(reg) addr.0, options(nostack, preserves_flags));
    }
}

/// Invalidate the TLB completely by reloading the CR3 register.
pub fn flush_all() {
    use registers::control_regs::{cr3, cr3_write};
    unsafe { cr3_write(cr3()) }
}
