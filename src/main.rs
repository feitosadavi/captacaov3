#[macro_use]
extern crate lazy_static;
pub mod context;
pub mod modules;
pub mod util;
pub mod core;
pub mod global_event_emitter;
pub mod constants;

use std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}};

use global_event_emitter::EVENT_EMITTER;
use tokio::runtime::Runtime;

use self::core::{
	structs::{TargetMethod, Log, Progress, Post, Report, Error, Success},
	events::{LOG, PROGRESS, POST},
	targets::TARGETS,
	implementations::MessengerDispatcher,
	situtations::{ERROR, SUCCESS}
};
use self::util::asyncthread;

// use teloxide::{prelude::*, utils::command::BotCommands};

async fn start_posts_getter_bot(query: &str, target: &str) {
	let target_clone = target.to_owned();
	let query_clone = query.to_owned();

	asyncthread::spawn_async(async move {
		TARGETS.get(&target_clone.as_str())
			.unwrap()
			.get_posts(&query_clone.as_str())
			.await
			.expect("App Error");
	}).join().expect("Error on start bot");
}

async fn start_messenger_bot(links: Vec<String>, target: &str) {
	let target_clone = target.to_owned();
	let links_clone = links.to_owned();

	asyncthread::spawn_async(async move {
		TARGETS.get(&target_clone.as_str())
			.unwrap()
			.send_message(links_clone)
			.await
			.expect("App Error");
	}).join().expect("Error on start bot");
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

// extern crate pretty_env_logger;
// #[macro_use] extern crate log;

use pretty_env_logger;
use log;

#[tokio::main]
async fn main() {
	std::env::set_var("TELOXIDE_TOKEN", "6511336966:AAFPx-_Uvzy4WxFfaCgk4ZNmdywdY7rXYKg");

	// let mut report = Report {
	// 		success: vec![],
	// 		errors: vec![]
	// 	};
    // let success: Arc<Mutex<RefCell<Vec<Success>>>> = Arc::new(Mutex::new(RefCell::new(Vec::new())));
    // let errors: Arc<Mutex<RefCell<Vec<Error>>>> = Arc::new(Mutex::new(RefCell::new(Vec::new())));
    // let success_clone = success.clone();
    // let errors_clone = errors.clone();

    // EVENT_EMITTER.lock().unwrap().on("LOG", move |log: Log| {
		// 	match log.situation.as_str() {
		// 		"error" => {
		// 			let error = Error {
		// 				target: log.target.clone(),
		// 				description: log.description.clone(),
		// 				link: log.link.clone(),
		// 			};
		// 			errors_clone.lock().unwrap().borrow_mut().push(error);
		// 			let errors = errors_clone.lock().unwrap();
		// 			// errors.borrow_mut().push(error);
		// 		}
		// 		"success" => {
		// 			let success = Success {
		// 				target: log.target.clone(),
		// 				link: log.link.clone(),
		// 			};
		// 			success_clone.lock().unwrap().borrow_mut().push(success);
		// 			let mut success = success_clone.lock().unwrap();
		// 			// success.borrow_mut().push(success);
		// 		},
		// 		"finished" => {
		// 			println!("{:?}", success);
		// 			println!("{:?}", errors);
		// 		}
		// 		_ => println!(""),
		// 	}
    // });
	// HelloWorld::new().unwrap().run().unwrap();
	
	pretty_env_logger::init();
	log::info!("Starting throw dice bot...");

	let bot = Bot::from_env();

	Command::repl(bot, answer).await;
	
	// let link = String::from("https://pr.olx.com.br/regiao-de-curitiba-e-paranagua/autos-e-pecas/carros-vans-e-utilitarios/fusca-tsi-2013-baixa-km-1156487976?rec=a&lis=galeria_adview_relacionados_2020");
	// let links = vec![link.to_owned(),link.to_owned()];

	// start_messenger_bot(links, "olx").await;
	// let query = "https://www.olx.com.br/autos-e-pecas/carros-vans-e-utilitarios/porsche/boxster?q=s";
	// let selected_targets = ["olx"];
	
	// EVENT_EMITTER.lock().unwrap().on(PROGRESS, |progress: Progress| println!("{:?}", progress));

	// EVENT_EMITTER.lock().unwrap().on(POST, move |post: Post| {
	// 	Runtime::new().unwrap().block_on(async move {

	// 		start_messenger_bot(post.links, post.target.as_str()).await;
	// 	});
	// });
	
	// for selected_target in selected_targets {
	// 	start_posts_getter_bot(query, selected_target).await;
	// }
}

use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Estes são os comandos suportados:")]
enum Command {
    #[command(description = "mostra este texto.")]
    Ajuda,
    #[command(description = "realiza o login na Olx.")]
    Login(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
	match cmd {
		Command::Ajuda => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
		Command::Login(username) => {
			bot.send_message(msg.chat.id, "Prossiga com o Login no navegador que apareceu na tela do seu computador.").await?;
			start_authentication("olx").await;	
			bot.send_message(msg.chat.id, "Login realizado com sucesso!.").await?
		}
		Command::UsernameAndAge { username, age } => {
			bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
				.await?
		}
	};

	Ok(())
}