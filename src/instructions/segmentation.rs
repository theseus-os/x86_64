//! Provides functions to read and write segment registers.

use structures::gdt::SegmentSelector;
use core::arch::asm;

/// Reload code segment register.
/// Note this is special since we can not directly move
/// to %cs. Instead we push the new segment selector
/// and return value on the stack and use lretq
/// to reload cs and continue at 1:.
pub unsafe fn set_cs(sel: SegmentSelector) {

    #[cfg(target_arch="x86_64")]
    #[inline(always)]
    unsafe fn inner(sel: SegmentSelector) {
        asm!(
            "push {sel}",
            "lea {tmp}, [1f + rip]",
            "push {tmp}",
            "retfq",
            "1:",
            sel = in(reg) u64::from(sel.0),
            tmp = lateout(reg) _,
            options(preserves_flags),
        );
    }

    inner(sel)
}

/// Reload stack segment register.
pub unsafe fn load_ss(sel: SegmentSelector) {
    asm!("mov ss, {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
}

/// Reload data segment register.
pub unsafe fn load_ds(sel: SegmentSelector) {
    asm!("mov ds, {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
}

/// Reload es segment register.
pub unsafe fn load_es(sel: SegmentSelector) {
    asm!("mov es, {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
}

/// Reload fs segment register.
pub unsafe fn load_fs(sel: SegmentSelector) {
    asm!("mov fs, {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
}

/// Reload gs segment register.
pub unsafe fn load_gs(sel: SegmentSelector) {
    asm!("mov gs, {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
}

/// Returns the current value of the code segment register.
pub fn cs() -> SegmentSelector {
    let segment: u16;
    unsafe { asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags)); }
    SegmentSelector(segment)
}
