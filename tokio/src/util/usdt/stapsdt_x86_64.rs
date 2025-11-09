unsafe extern "C" {
    static mut __usdt_sema_tokio_task__details: u16;
}

#[inline(always)]
pub(super) fn task_details(task_id: u64, name: &str, file: &str, line: u32, col: u32) {
    #[inline(never)]
    fn task_details_inner(task_id: u64, name: &str, file: &str, line: u32, col: u32) {
        // add nul bytes
        let name0 = [name.as_bytes(), b"\0"].concat();
        let file0 = [file.as_bytes(), b"\0"].concat();

        unsafe {
            std::arch::asm!(
                "990:   nop",
                "
                        .pushsection .note.stapsdt, \"\", \"note\"
                        .balign 4
                        .4byte 992f-991f, 994f-993f, 3
                991:
                        .asciz \"stapsdt\"
                992:
                        .balign 4
                993:
                        .8byte 990b
                        .8byte _.stapsdt.base
                        .8byte __usdt_sema_tokio_task__details
                        .asciz \"tokio\"
                        .asciz \"task-details\"
                        .asciz \"8@%rdi 8@%rsi 8@%rdx 4@%ecx 4@%r8d\"
                994:
                        .balign 4
                        .popsection",
                in("rdi") task_id,
                in("rsi") name0.as_ptr() as usize,
                in("rdx") file0.as_ptr() as usize,
                in("rcx") line as usize,
                in("r8") col as usize,
                options(nomem, nostack, preserves_flags),
            );
        }
    }

    if unsafe { __usdt_sema_tokio_task__details } != 0 {
        task_details_inner(task_id, name, file, line, col);
    }
}

/// spawned == 0 for block_on
/// spawned == 1 for task
#[inline(always)]
pub(super) fn task_start(task_id: u64, spawned: u8, size: usize, original_size: usize) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-start\"
                    .asciz \"8@%rdi 1@%sil 8@%rdx 8@%rcx\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            in("rsi") spawned as usize,
            in("rdx") size,
            in("rcx") original_size,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline(always)]
pub(crate) fn task_poll_start(task_id: u64) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-poll-start\"
                    .asciz \"8@%rdi\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline(always)]
pub(crate) fn task_poll_end(task_id: u64) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-poll-end\"
                    .asciz \"8@%rdi\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline(always)]
pub(crate) fn task_terminate(task_id: u64) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-terminate\"
                    .asciz \"8@%rdi\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline(always)]
pub(crate) fn waker_clone(task_id: u64) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-waker-clone\"
                    .asciz \"8@%rdi\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline(always)]
pub(crate) fn waker_drop(task_id: u64) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-waker-drop\"
                    .asciz \"8@%rdi\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline(always)]
pub(crate) fn waker_wake(task_id: u64) {
    unsafe {
        std::arch::asm!(
            "990:   nop",
            "
                    .pushsection .note.stapsdt, \"\", \"note\"
                    .balign 4
                    .4byte 992f-991f, 994f-993f, 3
            991:
                    .asciz \"stapsdt\"
            992:
                    .balign 4
            993:
                    .8byte 990b
                    .8byte _.stapsdt.base
                    .8byte 0b
                    .asciz \"tokio\"
                    .asciz \"task-waker-wake\"
                    .asciz \"8@%rdi\"
            994:
                    .balign 4
                    .popsection",
            in("rdi") task_id,
            options(nomem, nostack, preserves_flags),
        );
    }
}

pub(crate) fn register_probes() -> std::io::Result<()> {
    // doesn't do anything, just needs to be linked into the binary.
    #[allow(named_asm_labels)]
    unsafe {
        std::arch::asm!(
            // First define the semaphores
            ".ifndef __usdt_sema_tokio_task__details
                    .pushsection .probes, \"aw\", \"progbits\"
                    .weak __usdt_sema_tokio_task__details
                    .hidden __usdt_sema_tokio_task__details
            __usdt_sema_tokio_task__details:
                    .zero 2
                    .type __usdt_sema_tokio_task__details, @object
                    .size __usdt_sema_tokio_task__details, 2
                    .popsection
            .endif",
            // Finally define (if not defined yet) the base used to detect prelink
            // address adjustments.
            ".ifndef _.stapsdt.base
                    .pushsection .stapsdt.base, \"aGR\", \"progbits\", .stapsdt.base, comdat
                    .weak _.stapsdt.base
                    .hidden _.stapsdt.base
            _.stapsdt.base:
                    .space 1
                    .size _.stapsdt.base, 1
                    .popsection
            .endif",
            options(nomem, nostack, preserves_flags),
        );
    }

    Ok(())
}
