use std::error::Error;

pub struct Sanitizor {}

#[derive(Debug)]
pub struct PageStats {
	pages_count: i16,
	posts_count: i16
}

impl Sanitizor {
	pub fn extract_page_stats_number(text: String) -> Result<PageStats, Box<dyn Error>> {
		let Some((ipp, tol)) = text.split_once("de") else {
			panic!("Error");
		};

		let itens_per_page = ipp.to_string().trim().replace("1 - ", "");
		let total_of_links: String = tol.to_string().chars().filter(|c| c.is_digit(10)).collect();

		let itens_per_page_parsed = itens_per_page.parse::<f32>()?;
		let posts_count = total_of_links.parse::<f32>()?;

		let pages_count = (posts_count/itens_per_page_parsed).ceil() as i16; 


		return Ok(PageStats {pages_count, posts_count: posts_count as i16});
	}
}