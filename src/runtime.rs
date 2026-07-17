//! Shared multi-thread Tokio runtime for Matrix (and any async work from the GTK thread).
//!
//! IRC already runs on its own thread+runtime. Matrix must not call `tokio::spawn` from
//! the GTK thread without a reactor — use `runtime::spawn` instead.

use std::future::Future;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Process-wide multi-thread runtime (lazy).
pub fn handle() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("boulderX-async")
            .build()
            .expect("failed to create Tokio runtime")
    })
}

/// Spawn a future on the shared runtime. Safe to call from the GTK thread.
pub fn spawn<F>(fut: F) -> tokio::task::JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    handle().spawn(fut)
}
