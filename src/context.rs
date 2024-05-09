use std::{error::Error, path::Path};

use playwright::{Playwright, api::{BrowserContext, Browser, StorageState, Viewport, BrowserType}};

use crate::constants::STORAGE_PATH;


pub struct Context {}

impl Context {
	pub async fn save_storage_state (context:BrowserContext) {
		let storage_state = context.storage_state().await.unwrap();
		let storage_state_json = serde_json::to_string(&storage_state).unwrap();
		let storage_file_path = Path::new(STORAGE_PATH);
		
		std::fs::write(storage_file_path, storage_state_json).unwrap(); // grava as alterações de estado no arquivo padrao
	}

	fn load_storage_state () -> StorageState {
		// Load the storage state from file
		let storage_file_path = Path::new(STORAGE_PATH);
		let storage_state: Option<StorageState> = match std::fs::read_to_string(storage_file_path) {
			Ok(content) => {
				return serde_json::from_str(&content).unwrap();
			},
			// se der erro é pq está vazio, então volta um documento 'vazio'
			Err(_) => Some(StorageState {cookies: None, origins: None})
		};
		return storage_state.unwrap();
	}

	fn get_browser_name(browser_name: BrowserName, playwright: &Playwright) -> BrowserType {
		match browser_name {
			BrowserName::Chrome => playwright.chromium(),
			BrowserName::Firefox => playwright.firefox()
		}
	}

	// #[tokio::main]
	pub async fn new(browser_name: BrowserName) -> Result<(BrowserContext, Browser, Playwright), Box<dyn Error>> {
		println!("Criando contexto..");
		let playwright = Playwright::initialize().await?;
		playwright.prepare()?; // Install browsers

		let browser_name = Self::get_browser_name(browser_name, &playwright);

		let browser = browser_name.launcher().headless(true).launch().await?;

		let storage_state = Self::load_storage_state();
		let context = browser.context_builder().viewport(Some(Viewport {width: 1400, height: 600})).storage_state(storage_state).build().await?;

		Ok((context, browser, playwright))
	}
}

pub enum BrowserName {
	Firefox,
	Chrome
}