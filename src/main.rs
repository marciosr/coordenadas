#![windows_subsystem = "windows"]
mod main_window;
mod dialogo_cadastra_perfis;
mod frontend_data_check;
mod backend;

use crate::main_window::MainWindow;

fn main() {
	if gtk::init().is_err() {
    	println!("A inicialização do gtk falhou.");
    	return;
	}

	MainWindow::new().run();

  gtk::main();
}

