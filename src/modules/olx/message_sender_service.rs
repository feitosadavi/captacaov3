use std::{error::Error};
use crate::core::{
	implementations::MessengerDispatcher, 
	structs::{Progress, Log}
};


fn convert_usize_to_i32 (usize: usize) -> i32 {
	let i32 = match usize.try_into() {
		Ok(value) => value,
		Err(_) => panic!("Conversion from usize to i32 failed"),
	};
	return i32
}


pub async fn start (links: Vec<String>) -> Result<(), Box<dyn Error>> {	
	MessengerDispatcher::log(Log {
		target: "olx".to_string(),
		situation: "info".to_string(),
		description: "Enviando mensagem".to_string()
	});

	let total: i32 = convert_usize_to_i32(links.len());
	
	let mut i = 1;
	for _link in links {
		MessengerDispatcher::inform_progress(Progress {target: "olx".to_owned(), current: i, total});
		i += 1;
	}
	// let (context, _browser, _playwright) = context::Context::new().await?;

	// let page = context.new_page().await?;

	// page
	// 	.goto_builder(query)
	// 	.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
	// 	.goto().await?;
	Ok(())
}