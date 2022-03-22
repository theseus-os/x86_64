//! Enable and disable hardware interrupts.
use core::arch::asm;

/// Enable hardware interrupts using the `sti` instruction.
pub unsafe fn enable() {
    asm!("sti", options(nomem, nostack));
}

/// Disable hardware interrupts using the `cli` instruction.
pub unsafe fn disable() {
    asm!("cli", options(nomem, nostack));
}

/// Generate a software interrupt.
/// This is a macro because the argument needs to be an immediate.
#[macro_export]
macro_rules! int {
    ( $x:expr ) => {
        {
            asm!("int {id}", id = const $x, options(nomem, nostack));
        }
    };
}

/// Cause a breakpoint exception by invoking the `int3` instruction.
pub fn int3() {
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
}
