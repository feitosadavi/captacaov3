use core::time;
use std::{error::Error, thread};

use crate::{
	core::{implementations::MessengerDispatcher, structs::Log, situtations::INFO}, 
	context::{self, BrowserName, Context},
	constants::OLX_AUTH_PATH
};


pub async fn start() -> Result<(), Box<dyn Error>> {
	MessengerDispatcher::log(Log {
		target: "olx".to_string(),
		situation: INFO.to_string(),
		description: "Fazendo login".to_string(),
		link: "".to_string()
	});
	
	let (context, _browser, _playwright) = Context::new(BrowserName::Firefox, false).await?;

	let page = context.new_page().await?;

	page
		.goto_builder(OLX_AUTH_PATH)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;
	println!("Page Auth");
	thread::sleep(time::Duration::from_secs(300));

	context::Context::save_storage_state(context).await;

	Ok(())
}