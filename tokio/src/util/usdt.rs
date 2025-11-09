cfg_rt! {
    use crate::util::trace::SpawnMeta;

    cfg_usdt! {
        use std::mem;

        #[cfg(all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64")))]
        #[path = "usdt/macos.rs"]
        pub(crate) mod usdt_impl;

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        #[path = "usdt/stapsdt_x86_64.rs"]
        pub(crate) mod usdt_impl;

        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        #[path = "usdt/stapsdt_aarch64.rs"]
        pub(crate) mod usdt_impl;

        #[cfg(not(any(
            all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64")),
            all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64")),
        )))]
        compile_error!(
            "The `usdt` feature is only currently supported on \
linux/macos, on `aarch64` and `x86_64`."
);

        #[inline]
        pub(crate) fn task<F>(task: F, kind: &'static str, meta: SpawnMeta<'_>, id: u64) -> F {
            usdt_impl::task_start(id, (kind == "task") as u8, mem::size_of::<F>(), meta.original_size);
            usdt_impl::task_details(
                id,
                meta.name.unwrap_or_default(),
                meta.spawned_at.0.file(),
                meta.spawned_at.0.line(),
                meta.spawned_at.0.column(),
            );

            task
        }
    }

    cfg_not_usdt! {
        #[inline]
        pub(crate) fn task<F>(task: F, _kind: &'static str, _meta: SpawnMeta<'_>, _id: u64) -> F {
            // nop
            task
        }
    }
}
