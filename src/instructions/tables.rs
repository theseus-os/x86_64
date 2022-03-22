//! Instructions for loading descriptor tables (GDT, IDT, etc.).

use structures::gdt::SegmentSelector;
use core::arch::asm;

/// A struct describing a pointer to a descriptor table (GDT / IDT).
/// This is in a format suitable for giving to 'lgdt' or 'lidt'.
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

/// Load GDT table.
pub unsafe fn lgdt(gdt: &DescriptorTablePointer) {
    asm!("lgdt [{}]", in(reg) gdt, options(readonly, nostack, preserves_flags));
}

/// Load IDT table.
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
}

/// Load the task state register using the `ltr` instruction.
pub unsafe fn load_tss(sel: SegmentSelector) {
    asm!("ltr {0:x}", in(reg) sel.0, options(nomem, nostack, preserves_flags));
}
