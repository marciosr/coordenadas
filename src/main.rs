#![windows_subsystem = "windows"]
#[macro_use]
mod utils;
mod main_window;
mod dialogo_cadastra_perfis;
mod frontend_data_check;
mod backend;


use crate::main_window::MainWindow;
use gtk::Application;
use gtk::prelude::*;

fn main() {
	// if gtk::init().is_err() {
 //    	println!("A inicialização do gtk falhou.");
 //    	return;
	// }
	let application = Application::new(Some("com.github.marciosr.coordenadas"),
		Default::default());

	application.connect_startup(move|app|{
		let mainwindow = MainWindow::new();
		let window = mainwindow.window.clone();
		app.add_window(&window);
		mainwindow.run();
	});


	let ret = application.run();
	std::process::exit(ret);
}

