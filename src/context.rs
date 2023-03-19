use std::{error::Error, path::Path};

use playwright::{Playwright, api::{BrowserContext, Browser, StorageState, Viewport}};


pub struct Context {}

impl Context {
	fn load_storage_state () -> StorageState {
		// Load the storage state from file
		let storage_file_path = Path::new("./storage-state.json");
		let storage_state: Option<StorageState> = match std::fs::read_to_string(storage_file_path) {
			Ok(content) => {
				return serde_json::from_str(&content).unwrap();
			},
			Err(_) => Some(StorageState {cookies: None, origins: None})
		};
		return storage_state.unwrap();
	}

	// #[tokio::main]
	pub async fn new() -> Result<(BrowserContext, Browser, Playwright), Box<dyn Error>> {
		println!("Criando contexto..");
		let playwright = Playwright::initialize().await?;
		playwright.prepare()?; // Install browsers

		let chromium = playwright.chromium();

		let browser = chromium.launcher().headless(false).launch().await?;

		let storage_state = Self::load_storage_state();
		// println!("{:?}", storage_state);
		let context = browser.context_builder().viewport(Some(Viewport {width: 500, height: 600})).storage_state(storage_state).build().await?;

		Ok((context, browser, playwright))
	}
}