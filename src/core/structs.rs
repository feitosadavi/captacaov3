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
	pub link: String,
	pub description: String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Post {
	pub target: String,
	pub links: Vec<String>,
}

#[derive(Debug)]
pub struct Error {
	pub target: String,
	pub link: String,
	pub description: String
}

#[derive(Debug)]
pub struct Success {
	pub target: String,
	pub link: String,
}

pub struct Report {
	pub errors: Vec<Error>,
	pub success: Vec<Success>,
}

pub struct MessengerDispatcherArgs{}

