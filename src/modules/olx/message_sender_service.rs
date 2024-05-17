use std::{env, error::Error, thread, time::Duration};

use playwright::api::Page;

use crate::{
	context::{self, BrowserName}, core::{
		implementations::MessengerDispatcher, situtations::{ERROR, SUCCESS}, structs::Log
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
	
	async fn type_message (&self, page: Page) -> Result<(), Box<dyn Error>> {
		println!("[olx/message_sender_service]: Typing message");
		thread::sleep(Duration::from_secs(5));
		let selector_builder = page
			.wait_for_selector_builder("textarea[placeholder='Digite uma mensagem...']")
			.timeout(10000.0);
		
		match selector_builder.wait_for_selector().await  {
			Ok(textarea) => {
				println!("{:?}", textarea);
				match textarea {
					Some(element) => {
						let message = match env::var("MESSAGE")  {
							Ok(id) => id,
							Err(err) => panic!("{}", err)
						};
						element.type_builder(message.as_str()).r#type().await?;
						element.press_builder("Enter").press().await?;
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
		thread::sleep(Duration::from_secs(10));
		match page.wait_for_selector_builder("[data-element='button_reply-chat']").timeout(10000.0).wait_for_selector().await {
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
	
	pub async fn start(&mut self, links: Vec<String>) -> Result<i32, Box<dyn Error>>{
		println!("[olx/message_sender_service]: Start Sending Messages");
		
		let (context, _browser, _playwright) = context::Context::new(BrowserName::Firefox, true).await?;

		context.add_init_script("disable_css_and_image.js").await?;

		let page = context.new_page().await?;
		
		for link in links {
			self.link = link;

			page
				.goto_builder(self.link.as_str())
				.wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
				.goto().await?;
				self.click_chat_btn(page.to_owned()).await?;
				self.type_message(page.to_owned()).await?;

				println!("LOG");
				MessengerDispatcher::log(Log {
					situation: SUCCESS.to_string(), 
					target: "olx".to_string(), 
					description: "".to_string(),
					link: format!("Enviado para: {}", self.link)
				});	
		}
		Ok(10)
	}
	
}