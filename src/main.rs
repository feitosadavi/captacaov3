#[macro_use]
extern crate lazy_static;
pub mod context;
pub mod modules;
pub mod util;
pub mod core;
pub mod global_event_emitter;
pub mod constants;

use std::env;

use constants::CHAT_ID_ENV;
use global_event_emitter::EVENT_EMITTER;
use modules::olx::posts_getter_service;
use tokio::runtime::Runtime;

use self::core::{
	structs::{TargetMethod, Log, Post},
	events::{LOG, POST},
	targets::TARGETS,
	implementations::MessengerDispatcher,
	situtations::{ERROR, SUCCESS}
};
use self::util::asyncthread;

// async fn start_posts_getter_bot(query: &str, target: &str) {
// 	let target_clone = target.to_owned();
// 	let query_clone = query.to_owned();

// 	asyncthread::spawn_async(async move {
// 		TARGETS.get(&target_clone.as_str())
// 			.unwrap()
// 			.get_posts(&query_clone.as_str())
// 			.await
// 			.expect("App Error");
// 	}).join().expect("Error on start bot");
// }

async fn start_messenger_bot(links: Vec<String>, target: &str) {
	let target_clone = target.to_owned();
	let links_clone = links.to_owned();

	// asyncthread::spawn_async(async move {
		TARGETS.get(&target_clone.as_str())
			.unwrap()
			.send_message(links_clone)
			.await
			.expect("App Error");
	// }).join().expect("Error on start bot");
}

async fn start_authentication(target: &str) {
	let target_clone = target.to_owned();
 
	asyncthread::spawn_async(async move {
		let res = TARGETS.get(&target_clone.as_str())
			.unwrap()
			.authenticate()
			.await;

		match res {
			Err(_) => MessengerDispatcher::log(Log { 
				target: target_clone.to_string(), 
				situation: ERROR.to_string(), 
				description: "Já está autenticado".to_string(),
    		link: "".to_string(), 
			}),
    	Ok(_) => println!("OK"),
		}
	}).join().expect("Error on start authentication");
}

#[tokio::main]
async fn main() {
	println!("THREAD POST");
	EVENT_EMITTER.lock().unwrap().on(POST, move |post: Post| {
		println!("{:?}", post);
		Runtime::new().unwrap().block_on(async move {
			start_messenger_bot(post.links, post.target.as_str()).await;
		});
	});
	println!("THREAD LOG");
  EVENT_EMITTER.lock().unwrap().on(LOG, move |log: Log| {
		println!("{:?}", log);
		Runtime::new().unwrap().block_on(async move {
			match log.situation.as_str() {
				"success" => {
					let chat_id = match env::var(CHAT_ID_ENV)  {
						Ok(id) => id,
						Err(err) => panic!("{}", err)
					};
					let envbot: Bot = Bot::from_env();
					let _  = envbot.send_message(chat_id.clone(), log.link).await;
				},
				_ => println!(""),
			}		
		});
	});

	EVENT_EMITTER.lock().unwrap().emit(LOG, Log {
		situation: SUCCESS.to_string(), 
		target: "olx".to_string(), 
		description: "".to_string(),
		link: "MICROSOFT.COM".to_string()
	});
		env::set_var("RUST_BACKTRACE", "full");
	std::env::set_var("TELOXIDE_TOKEN", "6511336966:AAFPx-_Uvzy4WxFfaCgk4ZNmdywdY7rXYKg");
	let bot = Bot::from_env();
	Command::repl(bot, answer).await;
}

use teloxide::{
	prelude::*, 
	utils::command::BotCommands,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Estes são os comandos suportados:")]
enum Command {
    #[command(description = "mostra este texto.")]
    Ajuda,
    #[command(description = "realiza o login na Olx.")]
    Login,
    #[command(description = "Envia a mensagem para os anunciantes.")]
    EnviarMensagem(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
	match cmd {
		Command::Ajuda => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
		Command::Login => {
			bot.send_message(msg.chat.id, "Prossiga com o Login no navegador que apareceu na tela do seu computador.").await?;
			start_authentication("olx").await;	
			bot.send_message(msg.chat.id, "Login realizado com sucesso!.").await?
		}
		Command::EnviarMensagem(_search) => {
			env::set_var(CHAT_ID_ENV, msg.chat.id.to_string());
			bot.send_message(msg.chat.id, "Coletando os dados.").await?;
			let _ = posts_getter_service::get_posts_links(&_search).await;
			println!("Terminou");
			bot.send_message(msg.chat.id, format!("Acompanhe o envio das mensagens em https://conta.olx.com.br/chats")).await?
		}
	};

	Ok(())
}