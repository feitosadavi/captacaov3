use std::{error::Error, path::Path, thread, time};

use playwright::api::BrowserContext;

use crate::{core::{implementations::MessengerDispatcher, structs::Log}, context::{self}};

async fn save_storage_state (context:BrowserContext) {
	let storage_state = context.storage_state().await.unwrap();
	let storage_state_json = serde_json::to_string(&storage_state).unwrap();
	let storage_file_path = Path::new("./storage-state.json");
	std::fs::write(storage_file_path, storage_state_json).unwrap();
}

pub async fn start() -> Result<(), Box<dyn Error>> {
	MessengerDispatcher::log(Log {
		target: "olx".to_string(),
		situation: "info".to_string(),
		description: "Coletando dados".to_string()
	});
	
	let (context, _browser, _playwright) = context::Context::new().await?;

	let page = context.new_page().await?;

	page
		.goto_builder("https://conta.olx.com.br/acesso/")
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	page.type_builer("input[type='email']", "davifeitosa.trab@gmail.com").delay(0.0).no_wait_after(true).timeout(2000.0).r#type().await?;
	page.type_builer("input[type='password']", "40028922dD$").delay(0.0).no_wait_after(true).timeout(2000.0).r#type().await?;

	let entrar_btn = &page.query_selector_all("text=Entrar").await?[3];
	entrar_btn.click_builder().click().await?;

	thread::sleep(time::Duration::from_secs(3));
	save_storage_state(context).await;


	Ok(())
}