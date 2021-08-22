extern crate gtk;
// extern crate gio;

use std::fs;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;
use std::path::PathBuf;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Entry, Text,
	ComboBoxText, Revealer, Label, FileChooserDialog, ResponseType};
use gtk::{gio, glib};
//use std::fs::File;
use std::io::BufReader;
use glib::clone;
use gio::File;

use crate::dialogo_cadastra_perfis::Cadastra;
use crate::frontend_data_check::Dados;
use crate::backend::*;


pub struct MainWindow {
	pub glade:							Builder,
	pub window:							ApplicationWindow,
	pub ent_latitude:				Entry,
	pub ent_longitude:			Entry,
	pub ent_saida:					Entry,
	pub bt_fechar:					Button,
	pub bt_run:							Button,
	pub bt_entrada:					Button,
	pub bt_saida:						Button,
	pub bt_fecha_notifica:	Button,
	pub rv_notifica:				Revealer,
	pub lb_notifica:				Label,
	pub cb_perfis:					ComboBoxText,
	pub bt_ad:							Button,
	pub bt_rm:							Button
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let glade_src = include_str!("main_window.glade");
		let glade = gtk::Builder::from_string(glade_src);

		get_widget!(glade, ApplicationWindow, window);
		get_widget!(glade, Entry, ent_latitude);
		get_widget!(glade, Entry, ent_longitude);
		get_widget!(glade, Entry, ent_saida);
		get_widget!(glade, Button, bt_fechar);
		get_widget!(glade, Button, bt_run);
		get_widget!(glade, Button, bt_entrada);
		get_widget!(glade, Button, bt_saida);
		get_widget!(glade, Button, bt_fecha_notifica);
		get_widget!(glade, Revealer, rv_notifica);
		get_widget!(glade, Label, lb_notifica);
		get_widget!(glade, ComboBoxText, cb_perfis);
		get_widget!(glade, Button, bt_ad);
		get_widget!(glade, Button, bt_rm);

let _teste = gtk::Entry::new();

		ent_latitude.set_sensitive(true);




		MainWindow {
			glade,
			window,
			ent_latitude,
			ent_longitude,
			ent_saida,
			bt_fechar,
			bt_run,
			bt_entrada,
			bt_saida,
			bt_fecha_notifica,
			rv_notifica,
			lb_notifica,
			cb_perfis,
			bt_ad,
			bt_rm
		}
	}

	pub fn run(self) {

		let perfis_serializados = match carrega_perfis() {
			Ok(perfis) => perfis,
			Err(e) => { println!("Erro ao carregar os perfis: {}", e);
					serializa_yaml(&popula_perfis())
				},
		};

		// Uso do Rc<RefCell<>> com o intuito de permitir a mutabilidade interna
		// ou seja, que o conteúdo deste container seja mudado. Há um borrow checker
		// no runtime, portanto há custo na execussão do código.

		let perfis: Rc<RefCell<_>> = Rc::new(RefCell::new(desserializa_yaml(perfis_serializados)));

		inicia_combo (&self.cb_perfis, &perfis);

		self.cb_perfis.set_id_column(1); // Garante que haja um perfil ativo, assim não havera o crash de unwrap() on None.
		self.cb_perfis.set_active(Some(0));

		let perfis = match self.cb_perfis.active_text() {
			Some(_ativo) => perfis,
			None =>	{
				let perfis_populados: Rc<RefCell<_>> = Rc::new(RefCell::new(popula_perfis()));
				inicia_combo (&self.cb_perfis, &perfis_populados);
				perfis_populados
			},
		};

		self.cb_perfis.set_active(Some(0));

		let nome_perfil = self.cb_perfis.active_text().unwrap(); // Possível problema de unwrap sobre None
		atualiza_campos(nome_perfil.to_string(), &self.ent_latitude, &self.ent_longitude, &perfis);

		let window = self.window.clone();
		let uri_entrada: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::new()));
		let uri_saida: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::new()));

		{
			let uri_clone = uri_entrada.clone();

			self.bt_entrada.connect_clicked(move|_|{
				println!("Teste do callback antes de criar filechooser");

				let file_chooser = FileChooserDialog::new(
					Some("Open File"),
					Some(&window),
					gtk::FileChooserAction::Open,
					&[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
				);

				let uri_clone2 = uri_clone.clone();

				file_chooser.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
				let caminho = match response {
					gtk::ResponseType::Ok => {
						let file = d.file().expect("Couldn't get file");
						print!("Conteúdo de file {:?}", d.file());
						let full_path = file.path().expect("Couldn't get file path");
						let file = std::fs::File::open (&full_path.as_path()).expect("Couldn't open file");
						Some(full_path)
					},
						_ => None,
					};
					d.close();

					*uri_clone2.borrow_mut() = caminho.unwrap_or(PathBuf::new());
					println!("TESTE 1 {:?}", &uri_clone2);
				});
				file_chooser.show();
			});
		}


		{
			let uri_clone = uri_saida.clone();
			let window = self.window.clone();

			self.bt_saida.connect_clicked(move|_|{
				println!("Teste do callback antes de criar filechooser");

				let file_chooser = FileChooserDialog::new(
					Some("Escolha o diretório para salvar"),
					Some(&window),
					gtk::FileChooserAction::SelectFolder,
					&[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
				);

				let uri_clone2 = uri_clone.clone();

				file_chooser.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
				let caminho = match response {
					gtk::ResponseType::Ok => {
						let file = d.file().expect("Couldn't get file");
						print!("Conteúdo de file {:?}", d.file());
						let full_path = file.path().expect("Couldn't get file path");
						let file = std::fs::File::open (&full_path.as_path()).expect("Couldn't open file");
						Some(full_path)
					},
						_ => None,
					};
					d.close();

					*uri_clone2.borrow_mut() = caminho.unwrap_or(PathBuf::new());
					println!("TESTE 1.1 {:?}", &uri_clone2);
				});
				file_chooser.show();
			});
		}

		// let uri_entrada: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::new()));
		// let uri_saida: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::new()));
		// let mut uri_saida_conf: PathBuf;


		{ // Bloco de execussão da busca

			// let bt_entrada_clone = self.bt_entrada.clone();
			// let bt_saida_clone = self.bt_saida.clone();
			let ent_latitude_clone = self.ent_latitude.clone();
			let ent_longitude_clone = self.ent_longitude.clone();
			let ent_saida_clone = self.ent_saida.clone();
			let rv_notifica_clone = self.rv_notifica.clone();

			let lb_notifica_clone = self.lb_notifica.clone();

			let uri_entrada_clone = uri_entrada.clone();
			let uri_saida_clone = uri_saida.clone();

			let uri_entrada_clone2 = uri_entrada.clone();
			let uri_saida_clone2 = uri_saida.clone();



			self.bt_run.connect_clicked(move |_| {

				//println!("\n\n** TESTE 2 {:?}", &uri_clone);

				if Dados::check(&*uri_entrada_clone.borrow(), &*uri_saida_clone.borrow(), &ent_latitude_clone, &ent_longitude_clone, &ent_saida_clone, &rv_notifica_clone, &lb_notifica_clone) {

				 	let dados = Dados::new(&uri_entrada_clone2, &uri_saida_clone2, &ent_latitude_clone, &ent_longitude_clone, &ent_saida_clone);
				 	let texto = fs::read_to_string(&*dados.uri_entrada.borrow());

				 	match texto {
				 		Ok(_)	=> {
						 	analisa_texto (	&*dados.uri_entrada.borrow(),
						 					&dados.uri_saida,
						 					dados.latitude,
						 					dados.longitude).expect("Não foi possível carregar o arquivo de texto");
				 		},
				 		Err(e)		=> {
				 			println!("Erro no processamento do texto: {}", e);
				 			lb_notifica_clone.set_label("A codificação do arquivo de entrada deve ser UTF-8!\nConverta-o em um editor de texto.");
				 			rv_notifica_clone.set_reveal_child(true);
				 		},
				 	}
				} else { println!("Faltam parâmetros!"); }
			});
		}

		{
			let rv_notifica_clone = self.rv_notifica.clone();
			self.bt_fecha_notifica.connect_clicked(move |_| {
				rv_notifica_clone.set_reveal_child(false);
			});
		}

		{
			let combo = self.cb_perfis.clone();
			let ent_1 = self.ent_latitude.clone();
			let ent_2 = self.ent_longitude.clone();
			let perfis_clone = perfis.clone();

			combo.connect_changed(move |cb| {
				match cb.active_text() {
					Some(_texto) => {
						let nome_perfil = cb.active_text().unwrap();
						atualiza_campos(nome_perfil.to_string(), &ent_1, &ent_2, &perfis_clone);
					},
					None => println!("Não há texto ativo"),

				}


			});
		}

		{
			let cb_perfis_clone = self.cb_perfis.clone();
			let perfis_clone0 = perfis.clone();
			let window_clone = self.window.clone();
			self.bt_ad.connect_clicked(move |_| {

				let cadastra = Cadastra::new();
				let cadastra_clone = cadastra.clone();
				let cb_perfis_clone2 = cb_perfis_clone.clone();

				cadastra.dialog.set_transient_for(Some(&window_clone));

				let perfis_clone1 = perfis_clone0.clone();
				cadastra.bt_preencher.connect_clicked(move|_|{

					if	&cadastra_clone.ent_dialog_latitude.text().to_string() == "" ||
						&cadastra_clone.ent_dialog_longitude.text().to_string() == "" ||
						&cadastra_clone.ent_dialog_perfil.text().to_string() == "" {
						} else {

							let nome_perfil =  &cadastra_clone.ent_dialog_perfil
																.text().to_string();

							adiciona_perfil (	nome_perfil.to_string(),
												&cadastra_clone.ent_dialog_latitude
																.text().to_string(),
												&cadastra_clone.ent_dialog_longitude
																.text().to_string(),
												&perfis_clone1
											);
							cb_perfis_clone2.append_text(nome_perfil);
							println!("Teste do botão fecha diálogo\nO nome do perfil dentro do closure do bt-fecha é: {}\n
								A expressão da latitude é {}\n
								A expressão da longitude é {}\n
								o conteúdo dos perfiles no closure é: {:?}",
								nome_perfil,
								&cadastra_clone.ent_dialog_latitude
																.text().to_string(),
								&cadastra_clone.ent_dialog_longitude
																.text().to_string(),
								&perfis_clone1
							);

							cadastra_clone.dialog.close();
					}
				});

				cadastra.dialog.show();

			});
		}

		{
			let perfis_clone = perfis.clone();
			self.window.connect_close_request(move |_| {
				let map = perfis_clone.borrow();
				match salva_perfis(serializa_yaml(&map)) {
					Ok(a) => a,
					Err(e) => println!("Erro ao salvar os perfis: {}", e),
				};
				glib::signal::Inhibit(false) // Não funciona no gtk4
			});
		}

		{
			let perfis_clone = perfis.clone();
			let win = self.window.clone();
			self.bt_fechar.connect_clicked(move |_| {
				let map = perfis_clone.borrow();
				match salva_perfis(serializa_yaml(&map)) {
					Ok(a) => a,
					Err(e) => println!("Erro ao salvar os perfis: {}", e),
				};
				win.destroy();
				glib::signal::Inhibit(false); // Não funciona no gtk4
			});
		}

		{
			let combo = self.cb_perfis.clone();
			let perfis_clone = perfis.clone();

			self.bt_rm.connect_clicked(move|_| {
				//remove_perfil(&combo, &perfis_clone);
				match combo.active_text() {
					Some(perfil_ativo) => {
						remove_perfil (perfil_ativo.to_string(), &perfis_clone);
						atualiza_combo (&combo, &perfis_clone);
					},
					None =>	{},
				}

			});
		}
	self.window.show();

	}
}

pub fn inicia_combo (	combo: &ComboBoxText,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>) {
	let map = perfis.borrow();
	for (key, _value) in map.iter() {
		combo.append_text(&key);
	}
}

pub fn atualiza_campos(	nome_perfil: String,
						ent_latitude: &Entry,
						ent_longitude: &Entry,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	set_entrys(&ent_latitude, &ent_longitude, &String::from(nome_perfil), perfis);
}

pub fn atualiza_combo (	combo: &ComboBoxText,
												perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {
	combo.remove_all();

	let map = perfis.borrow();

	for (key, _value) in map.iter() {
		combo.append_text(&key);
	}
}

pub fn set_entrys (	entry_latitude: &Entry,
										entry_longitude: &Entry,
										nome_perfil: &String,
										perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {
	let map = perfis.borrow();

	let expressoes = map.get(nome_perfil).unwrap();
	entry_latitude.set_text(&expressoes.latitude);
	entry_longitude.set_text(&expressoes.longitude);
}

pub fn adiciona_perfil (perfil_n: String,
						latitude_n: &String,
						longitude_n: &String,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	let expressoes = Expressoes { latitude: latitude_n.to_string(), longitude: longitude_n.to_string()};

	let mut map: RefMut<_> = perfis.borrow_mut();
	//println!("O conteúdo dos perfils dentro da função adiciona perfi é: {:?}", map); // Para testes!
	map.insert(perfil_n, expressoes);
}

