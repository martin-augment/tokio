cfg_rt! {
    use crate::util::trace::SpawnMeta;

    cfg_usdt! {
        use core::{
            pin::Pin,
            task::{Context, Poll},
        };
        use pin_project_lite::pin_project;
        use std::future::Future;

        #[inline]
        pub(crate) fn task<F>(task: F, kind: &'static str, meta: SpawnMeta<'_>, id: u64) -> Instrumented<F> {
            fn get_meta(spawn_meta: SpawnMeta<'_>, id: u64) -> (u64, &str, usize, &'static str, u32, u32) {
                (
                    id,
                    spawn_meta.name.unwrap_or_default(),
                    spawn_meta.original_size,
                    spawn_meta.spawned_at.0.file(),
                    spawn_meta.spawned_at.0.line(),
                    spawn_meta.spawned_at.0.column(),
                )
            }

            // usdt only supports up to 6 args, so we need to split this into two.
            if kind == "task" {
                probes::task__spawn!(|| get_meta(meta, id));
            } else if kind == "block_on" {
                probes::task__block_on!(|| get_meta(meta, id));
            }

            Instrumented {
                inner: task,
                task_id: id,
            }
        }

        pin_project! {
            #[derive(Debug, Clone)]
            pub(crate) struct Instrumented<F> {
                #[pin]
                inner: F,
                task_id: u64,
            }

            impl<F> PinnedDrop for Instrumented<F> {
                fn drop(this: Pin<&mut Self>) {
                    let this = this.project();
                    probes::task__terminate!(|| *this.task_id);
                }
            }
        }

        impl<F: Future> Future for Instrumented<F> {
            type Output = F::Output;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let this = self.project();
                probes::task__poll__start!(|| *this.task_id);
                let res = this.inner.poll(cx);
                probes::task__poll__end!(|| *this.task_id);
                res
            }
        }

        #[usdt::provider(provider = "tokio")]
        #[allow(non_snake_case)]
        pub(crate) mod probes {
            fn task__spawn(task_id: u64, name: &str, size: usize, file: &str, line: u32, col: u32) {}
            fn task__block_on(task_id: u64, name: &str, size: usize, file: &str, line: u32, col: u32) {}
            fn task__poll__start(task_id: u64) {}
            fn task__poll__end(task_id: u64) {}
            fn task__terminate(task_id: u64) {}

            fn task__waker__clone(task_id: u64) {}
            fn task__waker__wake(task_id: u64) {}
            fn task__waker__drop(task_id: u64) {}
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
