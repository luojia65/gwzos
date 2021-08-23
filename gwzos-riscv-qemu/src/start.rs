//! Boot-up module

extern "C" fn boot_hart_main(hart_id: usize, opaque: usize) {
    drop(hart_id);
    drop(opaque);
}

#[allow(unused)] // should remove 
/* This function should be called by function sbi_hart_start(hartid, **start_addr**, opaque)*/
unsafe extern "C" fn hart_join(_hart_id: usize, _opaque: usize) {
    asm!(
        // Load hart stack pointer
        "mv     sp, a1", // should there be more initialize parameters other than stack pointer?
        // Skip to main function of other harts
        "j      {hart_join_main}",
        hart_join_main = sym hart_join_main,
    )
}

#[allow(unused)] // should remove 
extern "C" fn hart_join_main() {

}

// Only one hart would use the boot stack; the hart 0 should initialize a memory allocator
// and then prepare executor stack for each upcoming hart.
const BOOT_STACK_SIZE: usize = 16 * 1024;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

// When a system starts from SBI implementation, all its initial available cores should jump to
// the same entry address in QEMU. The hart 0, or the boot hart, will handle with the procedure left
// to boot all the remaining harts. Other harts while started, should report to the program
// and stop. When initialization is finished by the boot hart, it will read the reported hart list
// and start all the harts available.

// Some other systems may consider hide any harts from initialization, and will not start them
// by default. If we know their hart id's in advance, the boot program can join these harts into
// worker pools by starting them using hart id, then point them to hart joining main program.

// It's a typical way to start all harts to the same address in SBI level, which is considered default
// in QEMU emulated machines. It is also possible to start harts with different addresses. 
// If we program kernel for these type of machines, these individual harts should process with their
// own programs; then if necessary, run the join function to give the current hart to the worker pool.

#[naked]
#[link_section = ".text.entry"] 
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    asm!(
    // in QEMU, for non-boot hart, should report its existence to the system
    "
    beqz    a0, 1f
    2: j 2b # todo
1:  ",
    // for boot hart, set sp
    "
    la      sp, {boot_stack}
    li      t0, {boot_stack_size}
    add     sp, sp, t0
    ",
    // for boot hart, jump to rust_main (absolute address)
    "j      {boot_hart_main}", 
    boot_stack_size = const BOOT_STACK_SIZE,
    boot_stack = sym BOOT_STACK, 
    boot_hart_main = sym boot_hart_main,
    options(noreturn))
}
