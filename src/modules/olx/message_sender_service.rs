use std::{error::Error};
use crate::core::{implementations::MessengerDispatcher, structs::Progress};


fn convert_usize_to_i32 (usize: usize) -> i32 {
	let i32 = match usize.try_into() {
		Ok(value) => value,
		Err(_) => panic!("Conversion from usize to i32 failed"),
	};
	return i32
}


pub async fn start (links: Vec<String>) -> Result<(), Box<dyn Error>> {	
	MessengerDispatcher::log("Enviando mensagem");

	let total: i32 = convert_usize_to_i32(links.len());
	
	for (i, _link) in links.iter().enumerate() {
		let current = convert_usize_to_i32(i);
		MessengerDispatcher::inform_progress(Progress {current, total})
	}


	// let (context, _browser, _playwright) = context::Context::new().await?;

	// let page = context.new_page().await?;

	// page
	// 	.goto_builder(query)
	// 	.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
	// 	.goto().await?;


	Ok(())
}