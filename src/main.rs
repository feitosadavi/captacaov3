use event_emitter_rs::EventEmitter;
use modules::olx;
use util::asyncthread;

pub mod context;
pub mod modules;
pub mod util;

// #[tokio::main]
fn main() {
	let query = "https://www.olx.com.br/autos-e-pecas/carros-vans-e-utilitarios/toyota/corona/estado-df?q=";

	let mut event_emitter = EventEmitter::new();

	event_emitter.on("event", |value: String| {
		println!("{:?}", value)
	});

	let handle = asyncthread::spawn_async(async move {
    let res = olx::posts_getter_service::start(query).await.expect("Error on PostsGetterService");
		event_emitter.emit("event", res);
	});
		
	handle.join().unwrap();
}