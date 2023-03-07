use std::{error::Error};

use playwright::{Playwright, api::{BrowserContext, Browser}};


pub struct Context {}

impl Context {
	// #[tokio::main]
	pub async fn new() -> Result<(BrowserContext, Browser, Playwright), Box<dyn Error>> {
		println!("Criando contexto..");
		let playwright = Playwright::initialize().await?;
		playwright.prepare()?; // Install browsers

		let chromium = playwright.chromium();

		let browser = chromium.launcher().headless(true).launch().await?;

		let context = browser.context_builder().build().await?;

		Ok((context, browser, playwright))
	}
}