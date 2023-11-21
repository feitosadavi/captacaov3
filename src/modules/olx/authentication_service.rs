use core::time;
use std::{error::Error, thread};

use crate::{
	core::{implementations::MessengerDispatcher, structs::Log, situtations::INFO}, 
	context,
	constants::{OLX_AUTH_PATH, EMAIL}
};


pub async fn start() -> Result<(), Box<dyn Error>> {
	MessengerDispatcher::log(Log {
		target: "olx".to_string(),
		situation: INFO.to_string(),
		description: "Fazendo login".to_string(),
		link: "".to_string()
	});
	
	let (context, _browser, _playwright) = context::Context::new().await?;

	let page = context.new_page().await?;

	page
		.goto_builder(OLX_AUTH_PATH)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	page.type_builer("input[type='email']", EMAIL).no_wait_after(true).r#type().await?;
	page.query_selector_all("text=Continuar")
	.await?
	.first()
	.unwrap()
	.click_builder()
	.click()
	.await?;

	// page.type_builer("input[type='password']", "CP204060").delay(2.0).no_wait_after(true).timeout(2000.0).r#type().await?;
	thread::sleep(time::Duration::from_secs(30));

	context::Context::save_storage_state(context).await;

	Ok(())
}