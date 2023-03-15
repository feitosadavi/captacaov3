#[macro_use]
extern crate lazy_static;

pub mod context;
pub mod modules;
pub mod util;
pub mod core;
pub mod global_event_emitter;

use global_event_emitter::EVENT_EMITTER;

use self::core::{
	events::{LOG, POST},
	structs::{TargetMethod},
	targets::TARGETS
};
use self::util::asyncthread;

async fn start_bot(query: &str, target: &str) {
	TARGETS.get(target)
		.unwrap()
		.get_posts(query)
		.await
		.expect("App Error");
}

#[tokio::main]
async fn main() {
	let query = "https://www.olx.com.br/autos-e-pecas/carros-vans-e-utilitarios/porsche/boxster?q=s";
	let selected_targets = ["olx"];

	EVENT_EMITTER.lock().unwrap().on(LOG, |value: String| println!("{:?}", value));
	EVENT_EMITTER.lock().unwrap().on(POST, |value: Vec<String>| println!("{:?}", value));
	
	let handle = asyncthread::spawn_async(async move {
		for selected_target in selected_targets {
			if TARGETS.contains_key(selected_target) {
				start_bot(query, selected_target).await;
			}
		}
	});
		
	handle.join().expect("App Error");
}