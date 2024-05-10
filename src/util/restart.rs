use std::process::{exit, Command};

pub fn restart_program() -> ! {
	println!("Reiniciando o programa...");
	
	let status = Command::new(env!("CARGO"))
			.arg("run")
			.status()
			.expect("Falha ao reiniciar o programa");

	if !status.success() {
			eprintln!("Erro ao reiniciar o programa");
			exit(1);
	}

	// Este ponto nunca é alcançado, pois o programa será reiniciado
	unreachable!();
}