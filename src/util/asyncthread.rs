use std::thread::{self};

use std::future::Future;

pub fn spawn_async<F, T>(f: F) -> thread::JoinHandle<T> where
	F: Future<Output = T> + Send + 'static,
	T: Send + 'static,
{
    thread::spawn(move || {
			// Create a new Tokio runtime for the thread.
			let rt = tokio::runtime::Builder::new_multi_thread()
					.enable_all()
					.build()
					.unwrap();

			// Run the future on the runtime.
			rt.block_on(f)
    })
}
