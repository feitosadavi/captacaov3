use std::{error::Error, env};

use playwright::api::Page;
use teloxide::{Bot, requests::Requester, prelude::{Dispatcher, Dialogue}, types::{Update, Message}, dispatching::{dialogue::InMemStorage, UpdateFilterExt}};

use crate::{
	util::sanitizor::{Sanitizor, PageStats}, 
	context::{self, BrowserName}, 
	core::{implementations::MessengerDispatcher, 
	structs::{Log, Post, TelegramComunication}, 
	situtations::INFO, events::TELEGRAM_COMUNICATION}, 
	constants::{self, CHAT_ID_ENV}, global_event_emitter::EVENT_EMITTER
};

async fn get_number_of_pages(page: &Page) -> Result<PageStats, Box<dyn Error>>  {
	let option = page.query_selector(r#"p:has-text("resultados")"#).await?;
	
	let element = match option {
    Some(el) => el,
    None => {
			println!("num acho");
			panic!("Error")
		},
	};

	let text = element.inner_html().await?;
	let page_stats = Sanitizor::extract_page_stats_number(text)?;

	return Ok(page_stats);
}

async fn get_posts_from_current_page(page: &Page, url: &str) -> Result<Vec<String>, Box<dyn Error>>  {
	page
		.goto_builder(url)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	let anchor_elements = page.query_selector_all("a.olx-ad-card__link-wrapper").await?;

	let mut posts_links: Vec<String> = vec![];
	for element in &anchor_elements {
		let Some(href) = element.get_attribute("href").await? else {panic!("Error on getting attr")};
		posts_links.push(href);
	}
	
	Ok(posts_links)
}

pub async fn get_posts_links(query: &str, pages_count: i32, max_posts: usize) -> Result<Vec<String>, Box<dyn Error>> {
	let url = constants::OLX_SEARCH_URL.to_owned()+query;
	let (context, _browser, _playwright) = context::Context::new(BrowserName::Firefox).await?;
	let page = context.new_page().await?;
	page
		.goto_builder(&url)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	let mut total_links: Vec<String> = [].to_vec();
	let mut how_much_left = max_posts;
	for i in 0..pages_count {
		let url = [&url, "&o=", &(i+1).to_string()].join("");
		println!("{:?}", url);
		let links = get_posts_from_current_page(&page, &url).await?;
		for link in &links {
			total_links.push(link.to_string());
		}
		println!("{:?}", links.len());
		println!("{:?}", how_much_left);
		how_much_left = if how_much_left > links.len() {how_much_left - links.len()} else {0};
		if how_much_left == 0 {break};
	}
	// MessengerDispatcher::post(Post {
	// 	target: "olx".to_owned(),
	// 	links: total_links
	// });
	Ok(total_links)
}

pub async fn get_page_stats (query: &str) -> Result<PageStats, Box<dyn Error>> {	
	let chat_id = match env::var(CHAT_ID_ENV)  {
		Ok(id) => id,
		Err(err) => panic!("{}", err)
	};
	let envbot: Bot = Bot::from_env();
	envbot.send_message(chat_id.clone(), "Coletando os dados.").await?;

	let (context, _browser, _playwright) = context::Context::new(BrowserName::Firefox).await?;
	let page = context.new_page().await?;
	let url = constants::OLX_SEARCH_URL.to_owned()+query;
	page
		.goto_builder(&url)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	let page_stats = get_number_of_pages(&page).await?;

	
	Ok(page_stats)
}