use std::{error::Error, borrow::Borrow};

use playwright::{api::{Page}};

use crate::{util::sanitizor::{Sanitizor, PageStats}, context};

async fn get_number_of_pages(page: &Page) -> Result<PageStats, Box<dyn Error>>  {
	let xpath = "//span[contains(text(),'resultados')]";
	let element = page.wait_for_selector_builder(xpath).wait_for_selector().await?.unwrap();
	let text = element.inner_html().await?;
	let page_stats = Sanitizor::extract_page_stats_number(text)?;
	return Ok(page_stats);
}	

pub async fn start (query: &str) -> Result<(), Box<dyn Error>> {
	let (context, _browser, _playwright) = context::Context::new().await?;

	let page = context.new_page().await?;

	page
		.goto_builder(query)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;
	
	let page_stats = get_number_of_pages(page.borrow()).await?;
	println!("page_stats: {:?}", page_stats);

	Ok(())
}

