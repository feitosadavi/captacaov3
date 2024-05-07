use std::error::Error;

use teloxide::Bot;

use crate::{
	modules::olx, 
	core::events::{LOG}, 
	global_event_emitter::EVENT_EMITTER
};

use super::{
	structs::{TargetMethod, Progress, Log, Post, TelegramComunication}, 
	events::{POST, PROGRESS, TELEGRAM_COMUNICATION}
};

impl TargetMethod {
	// pub async fn get_posts(&self, query: &str) -> Result<(), Box<dyn Error>> {
	// 	match self.name {
	// 		"olx" => olx::posts_getter_service::start(query).await,
	// 		_ => panic!("Unsupported target method: {}", self.name)
	// 	}
	// }

	pub async fn send_message(&self, links: Vec<String>) -> Result<(), Box<dyn Error>> {
		match self.name {
			"olx" => {
				let mut message_sender = olx::message_sender_service::MessengerService { link: "".to_string() };
				message_sender.start(links).await
			},
			_ => panic!("Unsupported target method: {}", self.name)
		}
	}

	pub async fn authenticate(&self) -> Result<(), Box<dyn Error>> {
		match self.name {
			"olx" => Ok(olx::authentication_service::start().await?),
			_ => panic!("Unsupported target method: {}", self.name)
		}
	}
}

pub struct MessengerDispatcher {} 
impl MessengerDispatcher {
	pub fn log(value: Log) {
		EVENT_EMITTER.lock().unwrap().emit(LOG, value);
	}
	pub fn post(value: Post) {
		EVENT_EMITTER.lock().unwrap().emit(POST, value);
	}
	pub fn inform_progress(value: Progress) {
		EVENT_EMITTER.lock().unwrap().emit(PROGRESS, value);
	}
	pub fn telegram_comunication(value: TelegramComunication) {
		EVENT_EMITTER.lock().unwrap().emit(TELEGRAM_COMUNICATION, value);
	}
}
