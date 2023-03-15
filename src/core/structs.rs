use serde::{Serialize, Deserialize};

pub struct TargetMethod {
	pub name: &'static str
}

#[derive(Serialize, Deserialize)]
pub struct Progress {
	pub current: i32,
	pub total: i32
}

pub struct MessengerDispatcherArgs{}

