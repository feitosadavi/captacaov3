use serde::{Serialize, Deserialize};

pub struct TargetMethod {
	pub name: &'static str
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Progress {
	pub target: String,
	pub current: i32,
	pub total: i32
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Log {
	pub target: String,
	pub situation: String,
	pub description: String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Post {
	pub target: String,
	pub links: Vec<String>,
}

pub struct MessengerDispatcherArgs{}

