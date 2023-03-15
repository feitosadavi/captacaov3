use std::error::Error;

use crate::{
	modules::olx, 
	core::events::{LOG}, 
	global_event_emitter::EVENT_EMITTER
};

use super::{
	structs::{TargetMethod, Progress}, 
	events::{POST, PROGRESS}
};

impl TargetMethod {
	pub async fn get_posts(&self, query: &str) -> Result<(), Box<dyn Error>> {
		
		

		match self.name {
			"olx" => olx::posts_getter_service::start(query).await,
			_ => panic!("Unsupported target method: {}", self.name)
		}
	}

	pub async fn send_message(&self, links: Vec<String>) -> Result<(), Box<dyn Error>> {
		match self.name {
			"olx" => olx::message_sender_service::start(links).await,
			_ => panic!("Unsupported target method: {}", self.name)
		}
	}
}

pub struct MessengerDispatcher {} 
impl MessengerDispatcher {
	pub fn log(value: &str) {
		EVENT_EMITTER.lock().unwrap().emit(LOG, value);
	}
	pub fn post(value: Vec<String>) {
		EVENT_EMITTER.lock().unwrap().emit(POST, value);
	}
	pub fn inform_progress(value: Progress) {
		EVENT_EMITTER.lock().unwrap().emit(PROGRESS, value);
	}
}
