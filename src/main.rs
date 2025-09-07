#![windows_subsystem = "windows"]
#[macro_use]
mod utils;
mod backend;
mod dialogo_cadastra_perfis;
mod frontend_data_check;
mod main_window;

use crate::main_window::MainWindow;
use gtk::prelude::*;
use gtk::Application;

fn main() {
	let application = Application::new(Some("com.github.marciosr.coordenadas"), Default::default());

	application.connect_startup(move |app| {
		let mainwindow = MainWindow::new();
		let window = mainwindow.window.clone();
		app.add_window(&window);
		mainwindow.run();
	});

	let ret = application.run();
	std::process::exit(ret.into());
}
