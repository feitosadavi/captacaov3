use std::{error::Error, vec};

use playwright::{api::{Page, ElementHandle}};

use crate::{util::sanitizor::{Sanitizor, PageStats}, context};


async fn get_number_of_pages(page: &Page) -> Result<PageStats, Box<dyn Error>>  {
	let xpath = "//span[contains(text(),'resultados')]";
	let element = page.wait_for_selector_builder(xpath).wait_for_selector().await?.unwrap();
	let text = element.inner_html().await?;
	let page_stats = Sanitizor::extract_page_stats_number(text)?;
	return Ok(page_stats);
}

async fn get_posts_from_current_page(page: &Page) -> Result<Vec<String>, Box<dyn Error>>  {
	println!("Getting posts from current page");
	let selector = "a.sc-12rk7z2-1.huFwya.sc-gzVnrw.kGFTcZ";
	let anchor_elements = page.query_selector_all(selector).await?;

	let mut posts_links: Vec<String> = vec![];

	for element in &anchor_elements {
		let Some(href) = element.get_attribute("href").await? else {panic!("Error on getting attr")};
		posts_links.push(href);
	}
	
	println!("{:?}", posts_links);
	Ok(posts_links)
}

pub async fn start (query: &str) -> Result<&str, Box<dyn Error>> {
	let (context, _browser, _playwright) = context::Context::new().await?;

	let page = context.new_page().await?;

	page
		.goto_builder(query)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	// let page_stats = get_number_of_pages(page.borrow()).await?;

		// get_posts_from_current_page(&page).await?;

	// println!("page_stats: {:?}", page_stats);

	Ok("Finish")
}

