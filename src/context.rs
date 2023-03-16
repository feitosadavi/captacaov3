use std::{error::Error, path::Path};

use playwright::{Playwright, api::{BrowserContext, Browser, StorageState}};


pub struct Context {}

impl Context {
	fn load_storage_state () -> StorageState {
		// Load the storage state from file
		let storage_file_path = Path::new("./storage-state.json");
		let storage_state_json = std::fs::read_to_string(storage_file_path).unwrap();
		let storage_state: StorageState = serde_json::from_str(&storage_state_json).unwrap();
		return storage_state;
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
		let context = browser.context_builder().storage_state(storage_state).build().await?;

		Ok((context, browser, playwright))
	}
}