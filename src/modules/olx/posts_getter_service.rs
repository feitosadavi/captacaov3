use std::{error::Error};

use playwright::api::Page;

use crate::{util::sanitizor::{Sanitizor, PageStats}, context, core::{implementations::MessengerDispatcher, structs::{Log, Post}, situtations::INFO}};


async fn get_number_of_pages(page: &Page) -> Result<PageStats, Box<dyn Error>>  {
	let xpath = "//span[contains(text(),'resultados')]";
	let Ok(Some(element)) = page.wait_for_selector_builder(xpath).wait_for_selector().await else {panic!("Error")};
	let text = element.inner_html().await?;
	let page_stats = Sanitizor::extract_page_stats_number(text)?;
	return Ok(page_stats);
}

async fn get_posts_from_current_page(page: &Page, url: &str) -> Result<Vec<String>, Box<dyn Error>>  {
	println!("[olx/posts_getter_service]: Getting posts from current page");

	page
		.goto_builder(url)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	let selector = "a.sc-12rk7z2-1.huFwya.sc-gzVnrw.kGFTcZ";
	let anchor_elements = page.query_selector_all(selector).await?;

	let mut posts_links: Vec<String> = vec![];

	for element in &anchor_elements {
		let Some(href) = element.get_attribute("href").await? else {panic!("Error on getting attr")};
		posts_links.push(href);
	}
	
	Ok(posts_links)
}

pub async fn start (query: &str) -> Result<(), Box<dyn Error>> {	
	MessengerDispatcher::log(Log {
		target: "olx".to_string(),
		situation: INFO.to_string(),
		description: "Coletando dados".to_string(),
		link: "".to_string()
	});
	
	let (context, _browser, _playwright) = context::Context::new().await?;

	let page = context.new_page().await?;

	page
		.goto_builder(query)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	let page_stats = get_number_of_pages(&page).await?;

	for i in 0..page_stats.pages_count {
		let url = [query, "&o=", &(i+1).to_string()].join("");
		let links = get_posts_from_current_page(&page, &url).await?;
		MessengerDispatcher::post(Post {
			target: "olx".to_owned(),
			links
		});
	}
	
	Ok(())
}

