use std::{error::Error, thread, time::Duration};

use playwright::api::Page;

use crate::{
	constants::OLX_SEARCH_URL, context::{self, BrowserName}, core::{implementations::MessengerDispatcher, situtations::SUCCESS, structs::{Log, Post}}, util::sanitizor::{PageStats, Sanitizor} 
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

async fn scroll_to_bottom_smoothly(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
	loop {
			// Scroll down by a small amount
			page.eval("window.scrollBy(0, 3000)").await?;

			// Wait for a moment to let the content load and the scroll to take effect
			thread::sleep(Duration::from_millis(200));
			// Check if we've reached the bottom of the page
			let is_bottom: bool = page.eval::<bool>(r#"
					() => {
							const scrollTop = window.scrollY || window.pageYOffset;
							const scrollHeight = document.documentElement.scrollHeight;
							return scrollTop + window.innerHeight >= scrollHeight;
					}
			"#).await?;

			if is_bottom {
					break;
			}
	}

	Ok(())
}


async fn click_grade_view(page: &Page) -> Result<Option<()>, Box<dyn std::error::Error>> {
	// Encontre o elemento com o atributo aria-label igual a "Ativar visualização em grade"
	let selector = "[aria-label=\"Ativar visualização em grade\"]";
	let element = page.query_selector(selector).await?;
	
	match element {
			Some(element) => {
					element.click_builder().click().await?;
					Ok(Some(()))
			}
			None => {
					println!("Visualização em grade não encontrada.");
					Ok(None)
			}
	}
}

async fn get_posts_from_current_page(url: &str) -> Result<Vec<String>, Box<dyn Error>>  {
	let (context, _browser, _playwright) = context::Context::new(BrowserName::Firefox).await?;
	let page = context.new_page().await?;
	page.goto_builder(&url).goto().await?;

	click_grade_view(&page).await?;
	scroll_to_bottom_smoothly(&page).await?;

	let anchor_elements = page.query_selector_all("section.olx-ad-card.olx-ad-card--vertical:not(.rec-gallery-adcard)>a").await?;

	let mut posts_links: Vec<String> = vec![];
	for element in &anchor_elements {
		let href = match element.get_attribute("href").await? {
			Some(value) => value,
			None => {
					panic!("Error: href attribute not found");
			}
		};		
		posts_links.push(href);
	}

	Ok(posts_links)
}

pub async fn get_posts_links(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
	let url = OLX_SEARCH_URL.to_owned()+query;

	let (context, _browser, _playwright) = context::Context::new(BrowserName::Firefox).await?;
	let page = context.new_page().await?;
	page
		.goto_builder(&url)
		.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
		.goto().await?;

	println!("Getting page stats");
	let page_stats = get_number_of_pages(&page).await?;
	println!("{:?}", page_stats);
	page.close(Some(false)).await?;

	MessengerDispatcher::log(Log {
		situation: SUCCESS.to_string(), 
		target: "olx".to_string(), 
		description: "".to_string(),
		link: format!("Iniciando envio para {} anúncios. Acompanhe o resultado no chat da olx", page_stats.posts_count)
	});

	let mut total_links: Vec<String> = get_posts_from_current_page(&url).await?;
	MessengerDispatcher::post(Post {
		links: total_links.clone(),
		target: "olx".to_string()
	});
	for i in 1..page_stats.pages_count {
		let url = [&url, "&o=", &(i+1).to_string()].join("");
		let links = get_posts_from_current_page(&url).await?;
		total_links = [&total_links[..], &links[..]].concat();
		MessengerDispatcher::post(Post {
			links,
			target: "olx".to_string()
		});
	}
	println!("Total links: {:?}", total_links.len());
	Ok(total_links)
}
