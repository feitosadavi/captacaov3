use std::collections::HashMap;
use crate::TargetMethod;

lazy_static! {
	pub static ref TARGETS: HashMap<&'static str, TargetMethod> = {
		let mut targets: HashMap<&str, TargetMethod> = HashMap::new();
		targets.insert("olx", TargetMethod {name: "olx"});
		targets
	};
}