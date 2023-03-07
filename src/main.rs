use modules::olx;
use util::asyncthread;

pub mod context;
pub mod modules;
pub mod util;

// #[tokio::main]
fn main() {
	let query = "https://www.olx.com.br/autos-e-pecas/carros-vans-e-utilitarios/toyota/corona/estado-df?q=";

	let handle = asyncthread::spawn_async(async {
    olx::posts_getter_service::start(query).await.expect("Error on PostsGetterService");
	});
		
	handle.join().unwrap();
}