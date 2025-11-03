//! Details of link names:
//! * `*char`: `63686172202a`
//! * `u64`: `75696e7436345f74`
//! * `u32`: `75696e7433325f74`
//! * `u8`: `75696e74385f74`

unsafe extern "C" {
    #[allow(unused)]
    #[link_name = "__dtrace_stability$tokio$v1$1_1_0_1_1_0_1_1_0_1_1_0_1_1_0"]
    fn stability();

    #[allow(unused)]
    #[link_name = "__dtrace_typedefs$tokio$v2"]
    fn typedefs();

    #[link_name = "__dtrace_isenabled$tokio$task__details$v1"]
    fn task_details_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$task__details$v1$75696e7436345f74$63686172202a$63686172202a$75696e7433325f74$75696e7433325f74"]
    fn __task_details(
        task_id: core::ffi::c_ulonglong,
        name: *const core::ffi::c_char,
        file: *const core::ffi::c_char,
        line: core::ffi::c_uint,
        col: core::ffi::c_uint,
    );

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__start$v1"]
    fn task_start_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$task__start$v1$75696e7436345f74$75696e74385f74$75696e7436345f74$75696e7436345f74"]
    fn __task_start(
        task_id: core::ffi::c_ulonglong,
        spawned: core::ffi::c_uchar,
        size: core::ffi::c_ulonglong,
        original_size: core::ffi::c_ulonglong,
    );

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__poll__start$v1"]
    fn task_poll_start_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$task__poll__start$v1$75696e7436345f74"]
    fn __task_poll_start(task_id: core::ffi::c_ulonglong);

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__poll__end$v1"]
    fn task_poll_end_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$task__poll__end$v1$75696e7436345f74"]
    fn __task_poll_end(task_id: core::ffi::c_ulonglong);

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__terminate$v1"]
    fn task_terminate_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$task__terminate$v1$75696e7436345f74"]
    fn __task_terminate(task_id: core::ffi::c_ulonglong);

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__waker__clone$v1"]
    fn waker_clone_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$waker__clone$v1$75696e7436345f74"]
    fn __waker_clone(task_id: core::ffi::c_ulonglong);

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__waker__drop$v1"]
    fn waker_drop_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$waker__drop$v1$75696e7436345f74"]
    fn __waker_drop(task_id: core::ffi::c_ulonglong);

    #[allow(unused)]
    #[link_name = "__dtrace_isenabled$tokio$task__waker__wake$v1"]
    fn waker_wake_enabled() -> i32;

    #[allow(unused)]
    #[link_name = "__dtrace_probe$tokio$waker__wake$v1$75696e7436345f74"]
    fn __waker_wake(task_id: core::ffi::c_ulonglong);
}

#[cfg(target_arch = "aarch64")]
macro_rules! call_probe {
    ($fn:ident, $x0:expr $(,)?) => {
        ::std::arch::asm!(
            ".reference {typedefs}",
            "bl {extern_probe_fn}",
            ".reference {stability}",
            typedefs = sym typedefs,
            extern_probe_fn = sym $fn,
            stability = sym stability,
            in("x0") ($x0),
            options(nomem, nostack, preserves_flags)
        )
    };
    ($fn:ident, $x0:expr, $x1:expr, $x2:expr, $x3:expr $(,)?) => {
        ::std::arch::asm!(
            ".reference {typedefs}",
            "bl {extern_probe_fn}",
            ".reference {stability}",
            typedefs = sym typedefs,
            extern_probe_fn = sym $fn,
            stability = sym stability,
            in("x0") ($x0),
            in("x1") ($x1),
            in("x2") ($x2),
            in("x3") ($x3),
            options(nomem, nostack, preserves_flags)
        )
    };
    ($fn:ident, $x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr $(,)?) => {
        ::std::arch::asm!(
            ".reference {typedefs}",
            "bl {extern_probe_fn}",
            ".reference {stability}",
            typedefs = sym typedefs,
            extern_probe_fn = sym $fn,
            stability = sym stability,
            in("x0") ($x0),
            in("x1") ($x1),
            in("x2") ($x2),
            in("x3") ($x3),
            in("x4") ($x4),
            options(nomem, nostack, preserves_flags)
        )
    };
}

#[cfg(target_arch = "x86_64")]
macro_rules! call_probe {
    ($fn:ident, $x0:expr $(,)?) => {
        ::std::arch::asm!(
            ".reference {typedefs}",
            "call {extern_probe_fn}",
            ".reference {stability}",
            typedefs = sym typedefs,
            extern_probe_fn = sym $fn,
            stability = sym stability,
            in("rdi") ($x0),
            options(nomem, nostack, preserves_flags)
        )
    };
    ($fn:ident, $x0:expr, $x1:expr, $x2:expr, $x3:expr $(,)?) => {
        ::std::arch::asm!(
            ".reference {typedefs}",
            "call {extern_probe_fn}",
            ".reference {stability}",
            typedefs = sym typedefs,
            extern_probe_fn = sym $fn,
            stability = sym stability,
            in("rdi") ($x0),
            in("rsi") ($x1),
            in("rdx") ($x2),
            in("rcx") ($x3),
            options(nomem, nostack, preserves_flags)
        )
    };
    ($fn:ident, $x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr $(,)?) => {
        ::std::arch::asm!(
            ".reference {typedefs}",
            "call {extern_probe_fn}",
            ".reference {stability}",
            typedefs = sym typedefs,
            extern_probe_fn = sym $fn,
            stability = sym stability,
            in("rdi") ($x0),
            in("rsi") ($x1),
            in("rdx") ($x2),
            in("rcx") ($x3),
            in("r8") ($x4),
            options(nomem, nostack, preserves_flags)
        )
    };
}

#[inline(always)]
pub(super) fn task_details(task_id: u64, name: &str, file: &str, line: u32, col: u32) {
    #[inline(never)]
    fn task_details_inner(task_id: u64, name: &str, file: &str, line: u32, col: u32) {
        // add nul bytes
        let name0 = [name.as_bytes(), b"\0"].concat();
        let file0 = [file.as_bytes(), b"\0"].concat();

        unsafe {
            call_probe!(
                __task_details,
                task_id as usize,
                name0.as_ptr() as usize,
                file0.as_ptr() as usize,
                line as usize,
                col as usize,
            );
        }
    }

    if unsafe { task_details_enabled() } != 0 {
        task_details_inner(task_id, name, file, line, col);
    }
}

/// spawned == 0 for block_on
/// spawned == 1 for task
#[inline(always)]
pub(super) fn task_start(task_id: u64, spawned: u8, size: usize, original_size: usize) {
    unsafe {
        call_probe!(
            __task_start,
            task_id as usize,
            spawned as usize,
            size,
            original_size,
        );
    }
}

#[inline(always)]
pub(super) fn task_poll_start(task_id: u64) {
    unsafe { call_probe!(__task_poll_start, task_id as usize) }
}

#[inline(always)]
pub(super) fn task_poll_end(task_id: u64) {
    unsafe { call_probe!(__task_poll_end, task_id as usize) }
}

#[inline(always)]
pub(super) fn task_terminate(task_id: u64) {
    unsafe { call_probe!(__task_terminate, task_id as usize) }
}

#[inline(always)]
pub(crate) fn waker_clone(task_id: u64) {
    unsafe { call_probe!(__waker_clone, task_id as usize) }
}

#[inline(always)]
pub(crate) fn waker_drop(task_id: u64) {
    unsafe { call_probe!(__waker_drop, task_id as usize) }
}

#[inline(always)]
pub(crate) fn waker_wake(task_id: u64) {
    unsafe { call_probe!(__waker_wake, task_id as usize) }
}

pub(crate) fn register_probes() -> std::io::Result<()> {
    Ok(())
}
