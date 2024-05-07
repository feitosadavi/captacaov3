#[macro_use]
extern crate lazy_static;
pub mod context;
pub mod modules;
pub mod util;
pub mod core;
pub mod global_event_emitter;
pub mod constants;

use std::{env, thread};

use constants::CHAT_ID_ENV;
use global_event_emitter::EVENT_EMITTER;
use modules::olx::posts_getter_service;
use tokio::runtime::Runtime;

use crate::modules::olx::{self, message_sender_service};

use self::core::{
	structs::{TargetMethod, Log, Post},
	events::{LOG, PROGRESS, POST},
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
	// thread::spawn(|| {
		println!("THREAD");
		EVENT_EMITTER.lock().unwrap().on(POST, move |post: Post| {
			println!("POST EVENT");
			EVENT_EMITTER.lock().unwrap().emit("SEND_MSG", post);
		});
	// });
	println!("THREAD MENSAGEM");
	
	EVENT_EMITTER.lock().unwrap().on("SEND_MSG", move |post: Post| {
		println!("{:?}", post);
		Runtime::new().unwrap().block_on(async move {
			start_messenger_bot(post.links, post.target.as_str()).await;
		});
	});
	println!("THREAD LOG");
  EVENT_EMITTER.lock().unwrap().on("LOG", move |log: Log| {
		println!("{:?}", log);
		Runtime::new().unwrap().block_on(async move {
			match log.situation.as_str() {
				"success" => {
					let chat_id = match env::var(CHAT_ID_ENV)  {
						Ok(id) => id,
						Err(err) => panic!("{}", err)
					};
					let envbot: Bot = Bot::from_env();
					envbot.send_message(chat_id.clone(), format!("Enviado para: {}", log.link)).await;
				},
				_ => println!(""),
			}		
		});
	});

	// EVENT_EMITTER.lock().unwrap().emit("SEND_MSG", Post {links: 
		// ["https://df.olx.com.br/distrito-federal-e-regiao/autos-e-pecas/carros-vans-e-utilitarios/tesla-model-s-plaid-eletrico-1246870935?lis=listing_no_category".to_string()].to_vec(), target: "olx".to_string()});

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
		#[command(description = "A quantidade de anúncios que quer alcançar.")]
    Qtd(String)
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
			
			let stats = match posts_getter_service::get_page_stats(&_search).await {
				Ok(stats) => stats,
				Err(_err) => panic!("")
			};
			println!("Stats: {:?}", stats);

			env::set_var("SEARCH", &_search);
			env::set_var("PAGES_COUNT", stats.pages_count.to_string());
			env::set_var("POSTS_COUNT", stats.posts_count.to_string());

			bot.send_message(msg.chat.id, 
				format!("Encontrei {} anúncios, para quantos você deseja enviar? (informe com o comando /qtd)", stats.posts_count)).await?
		}
		Command::Qtd(_qtd) => {
			let sent_qtd: i32 = _qtd.parse().unwrap();
			let pages_count = env::var("PAGES_COUNT").unwrap().parse().unwrap();
			let posts_count: i32 = env::var("POSTS_COUNT").unwrap().parse().unwrap();
			let search = env::var("SEARCH").unwrap();
			println!("Sent qtd: {:?}", sent_qtd);
			println!("pages_count: {:?}", pages_count);
			println!("posts_count: {:?}", posts_count);
			if sent_qtd > posts_count {
				bot.send_message(msg.chat.id, "O número de posts a enviar deve ser menor que o total de posts encontrados.").await?;
			} else {
				bot.send_message(msg.chat.id, "Ok!").await?;
				let links = match posts_getter_service::get_posts_links(&search, pages_count, sent_qtd.try_into().unwrap()).await {
					Ok(links) => links,
					Err(_err) => panic!("")
				};
				let mut message_sender = olx::message_sender_service::MessengerService { link: "".to_string() };
				message_sender.start(links).await.unwrap();
			}
			bot.send_message(msg.chat.id, "Ok!").await?
		}
	};

	Ok(())
}