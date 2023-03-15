#[macro_use]
extern crate lazy_static;
pub mod context;
pub mod modules;
pub mod util;
pub mod core;
pub mod global_event_emitter;

use global_event_emitter::EVENT_EMITTER;
use tokio::runtime::{Runtime};

use self::core::{
	events::{LOG, POST, PROGRESS},
	structs::{TargetMethod, Progress, Log, Post},
	targets::TARGETS
};
use self::util::asyncthread;

async fn start_posts_getter_bot(query: &str, target: &str) {
	let target_clone = target.to_owned();
	let query_clone = query.to_owned();

	asyncthread::spawn_async(async move {
		TARGETS.get(&target_clone.as_str())
			.unwrap()
			.get_posts(&query_clone.as_str())
			.await
			.expect("App Error");
	}).join().expect("Error on start bot");
}

async fn start_messenger_bot(links: Vec<String>, target: &str) {
	let target_clone = target.to_owned();
	let links_clone = links.to_owned();

	asyncthread::spawn_async(async move {
		TARGETS.get(&target_clone.as_str())
			.unwrap()
			.send_message(links_clone)
			.await
			.expect("App Error");
	}).join().expect("Error on start bot");
}
 

#[tokio::main]
async fn main() {
	let query = "https://www.olx.com.br/autos-e-pecas/carros-vans-e-utilitarios/porsche/boxster?q=s";
	let selected_targets = ["olx"];

	EVENT_EMITTER.lock().unwrap().on(PROGRESS, |progress: Progress| println!("{:?}", progress));
	EVENT_EMITTER.lock().unwrap().on(LOG, |log: Log| println!("{:?}", log));

	EVENT_EMITTER.lock().unwrap().on(POST, move |post: Post| {
		Runtime::new().unwrap().block_on(async move {
			start_messenger_bot(post.links, post.target.as_str()).await;
		});
	});
	
	for selected_target in selected_targets {
		start_posts_getter_bot(query, selected_target).await;
	}

	// let handle = asyncthread::spawn_async(async move {
	// });
		
	// handle.join().expect("App Error");
}