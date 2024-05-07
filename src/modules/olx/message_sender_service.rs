use std::{env, error::Error, path, thread, time::Duration};

use playwright::api::{Page, BrowserContext};
use teloxide::{requests::Requester, Bot};

use crate::{
	constants::CHAT_ID_ENV, context::{self, BrowserName}, core::{
		implementations::MessengerDispatcher, situtations::{ERROR, FINISHED, SUCCESS}, structs::Log
	}
};

pub struct MessengerService {pub link: String}
impl MessengerService {
	fn log_error(&self, err_msg: &str) {
		MessengerDispatcher::log(Log { 
			target: "olx".to_string(),
			situation: ERROR.to_string(),
			description: err_msg.to_string(),
			link: self.link.to_owned()
		})
	}

	async fn click_send_btn(&self, page: Page) -> Result<(), Box<dyn Error>>{
		println!("[olx/message_sender_service]: Clicking Send Button");
		page.press_builder("#input-text-message", "Enter").delay(0.0).timeout(300.0).no_wait_after(false).press().await?;
		// page.keyboard().press("Enter").unwrap();
		// let selector_builder = page
		// 	.wait_for_selector_builder("div[aria-label='Enviar mensagem']")
		// 	.timeout(9000.0);
		// match selector_builder.wait_for_selector().await {
		// 	Ok(send_btn) => {
		// 		println!("{:?}", send_btn);
		// 		// send_btn.unwrap().click_builder().click().await?
		// 	},
		// 	Err(_) => self.log_error("Send Button Not Found"),		
		// };
	
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
		eprintln!("{}", msg);
	
		Ok(has_sent)
	}
	
	async fn type_message (&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Typing message");
		let selector_builder = page
			.wait_for_selector_builder("textarea[placeholder='Digite uma mensagem...']")
			.timeout(9000.0);
		
		match selector_builder.wait_for_selector().await  {
			Ok(textarea) => {
				println!("{:?}", textarea);
				match textarea {
					Some(element) => {
						element.type_builder("TEXTO").r#type().await?;
						// element.press_builder("Enter").press().await?;
					}
					None => self.log_error("Textarea Not Found")
				}
			},
			Err(_) => self.log_error("Textarea Not Found"),
		}
	
		Ok(())
	}
	
	async fn click_chat_btn (&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Clicking Chat Button");
		thread::sleep(Duration::from_secs(3));
		match page.wait_for_selector_builder("[data-element='button_reply-chat']").timeout(3000.0).wait_for_selector().await {
			Ok(chat_btns) => {
				match chat_btns {
					Some(btn) => {
						println!("{:?}", btn);
						btn.click_builder().click().await?;
						println!("clicked");
					},
					None => self.log_error("Chat Button Not Found"),
				}
			},
			Err(_) => self.log_error("Chat Button Not Found"),
		};
	
		Ok(())
	}
	
	async fn click_cookies_button(&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Clicking Cookies Button");	
		match page.query_selector("#cookie-notice-ok-button").await {
			Ok(button) => {
				button.unwrap().click_builder().click().await?;
			},
			Err(_) => self.log_error("Cookies Button Not Found"),
		}
		Ok(())
	}
	
	pub async fn start(&mut self, links: Vec<String>) -> Result<(), Box<dyn Error>>{
		println!("[olx/message_sender_service]: Start Sending Messages");
		
		let (context, browser, _playwright) = context::Context::new(BrowserName::Firefox).await?;

		context.add_init_script("disable_css_and_image.js").await?;

		let page = context.new_page().await?;
		
		let mut i = 0;
		for link in links {
			self.link = link;

			page
				.goto_builder(self.link.as_str())
				.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
				.goto().await?;
	
			// if i == 1 {
			// 	self.click_cookies_button(page.to_owned()).await?;
			// }
			
			// if i != 1 {
				self.click_chat_btn(page.to_owned()).await?;
				// if !self.has_sent_previous_messages(page.to_owned()).await? {
					self.type_message(page.to_owned()).await?;
					// self.click_send_btn(page.to_owned()).await?;
					// let chat_id = match env::var(CHAT_ID_ENV)  {
					// 	Ok(id) => id,
					// 	Err(err) => panic!("{}", err)
					// };
					// let envbot: Bot = Bot::from_env();
					// envbot.send_message(chat_id.clone(), format!("Enviado para: {}", self.link.to_string())).await?;

					// println!("lOGGING");
					// MessengerDispatcher::log(Log {
					// 	situation: SUCCESS.to_string(), 
					// 	target: "olx".to_string(), 
					// 	description: "".to_string(),
					// 	link: self.link.to_string()
					// })
				// }
			// }
	
			i += 1;
		}

		let chat_id = match env::var(CHAT_ID_ENV)  {
			Ok(id) => id,
			Err(err) => panic!("{}", err)
		};
		let envbot: Bot = Bot::from_env();
		envbot.send_message(chat_id.clone(), format!("Enviado para {} anúncios", i)).await?;
		Ok(())
	}
	
}