use std::error::Error;

use playwright::api::Page;

use crate::{
	core::{
		implementations::MessengerDispatcher, 
		structs::Log
	}, 
		context
};

pub struct MessengerService {pub link: String}
impl MessengerService {
	async fn click_send_btn(&self, page: Page) -> Result<(), Box<dyn Error>>{
		println!("[olx/message_sender_service]: Clicking Send Button");
		let selector_builder = page
			.wait_for_selector_builder("div[aria-label='Enviar mensagem']")
			.timeout(9000.0);
		match selector_builder.wait_for_selector().await {
			Ok(send_btn) => {
				send_btn.unwrap().click_builder().click().await?
			},
			Err(_) => {
				let err_msg = "Send Button Not Found";
				println!("{}", err_msg);
				MessengerDispatcher::log(Log { target: "olx".to_string(), situation: "error".to_string(), description: err_msg.to_string() })
				
			},
		};
	
		Ok(())
	}

	async fn has_sent_previous_messages (&self, page: Page) -> Result<bool, Box<dyn Error>> {
		println!("[olx/message_sender_service]: Checking if previous messages was sent");
		let selector_builder = page.wait_for_selector_builder("div[type='Alone']")
		.timeout(9000.0);
		
		let has_sent = match selector_builder.wait_for_selector().await  {
			Ok(_) => true,
			Err(_) => false,
		};
		let msg = if has_sent {"Has Sent Previous Messages"} else {"Has Not Sent Previous Messages"};
		println!("{}", msg);
	
		Ok(has_sent)
	}
	
	async fn type_message (&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Typing message");
		let selector_builder = page.wait_for_selector_builder("textarea[placeholder='Digite uma mensagem...']")
		.timeout(9000.0);
		
		match selector_builder.wait_for_selector().await  {
			Ok(textarea) => textarea.unwrap().type_builder("TEXTO").r#type().await?,
			Err(_) => eprintln!("Textarea Not Found"),
		}
	
		Ok(())
	}
	
	async fn click_chat_btn (&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Clicking Chat Button");
		match page.query_selector_all("span:has-text('Chat'):last-child").await {
			Ok(chat_btns) => {
				chat_btns.last().unwrap().click_builder().click().await?
			},
			Err(_) => {
				println!("Chat Button Not Found")
			},
		};
	
		Ok(())
	}
	
	async fn click_cookies_button(&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Clicking Cookies Button");	
		match page.query_selector("#cookie-notice-ok-button").await {
			Ok(button) => {
				button.unwrap().click_builder().click().await?;
			},
			Err(_) => {
				let err_msg = "Cookies Button Not Found";
				println!("{}", err_msg);
				MessengerDispatcher::log(Log { target: "olx".to_string(), situation: "error".to_string(), description: err_msg.to_string() })

			},
		}
		Ok(())
	}
	
	pub async fn start(&mut self, links: Vec<String>) -> Result<(), Box<dyn Error>>{
		println!("[olx/message_sender_service]: Start Sending Messages");
	
		let (context, _browser, _playwright) = context::Context::new().await?;
		let page = context.new_page().await?;
	
		
		let mut i = 1;
		for link in links {
			self.link = link;

			page
				.goto_builder(self.link.as_str())
				.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
				.goto().await?;
	
			if i == 1 {
				self.click_cookies_button(page.to_owned()).await?;
			}
			
			if i != 1 {
				self.click_chat_btn(page.to_owned()).await?;
				if !self.has_sent_previous_messages(page.to_owned()).await? {
					self.type_message(page.to_owned()).await?;
					self.click_send_btn(page.to_owned()).await?;
				}
			}
	
			i += 1;
		}
	
		Ok(())
	}
	
}