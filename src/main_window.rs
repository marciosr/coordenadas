extern crate gtk;

use gtk::glib::{self, Propagation};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, DropDown, Entry, Label, Revealer, StringList};
use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::backend::*;
use crate::dialogo_cadastra_perfis::Cadastra;
use crate::frontend_data_check::Dados;

pub struct MainWindow {
	pub ui: Builder,
	pub window: ApplicationWindow,
	pub ent_latitude: Entry,
	pub ent_longitude: Entry,
	pub ent_planilha: Entry,
	pub bt_fechar: Button,
	pub bt_run: Button,
	pub bt_entrada: Button,
	pub bt_saida: Button,
	pub bt_fecha_notifica: Button,
	pub rv_notifica: Revealer,
	pub lb_notifica: Label,
	pub cb_perfis: DropDown,
	pub bt_ad: Button,
	pub bt_rm: Button,
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let ui_src = include_str!("main_window.ui");
		let ui = gtk::Builder::from_string(ui_src);

		get_widget!(ui, ApplicationWindow, window);
		get_widget!(ui, Entry, ent_latitude);
		get_widget!(ui, Entry, ent_longitude);
		get_widget!(ui, Entry, ent_planilha);
		get_widget!(ui, Button, bt_fechar);
		get_widget!(ui, Button, bt_run);
		get_widget!(ui, Button, bt_entrada);
		get_widget!(ui, Button, bt_saida);
		get_widget!(ui, Button, bt_fecha_notifica);
		get_widget!(ui, Revealer, rv_notifica);
		get_widget!(ui, Label, lb_notifica);
		get_widget!(ui, DropDown, cb_perfis);
		get_widget!(ui, Button, bt_ad);
		get_widget!(ui, Button, bt_rm);

		MainWindow {
			ui,
			window,
			ent_latitude,
			ent_longitude,
			ent_planilha,
			bt_fechar,
			bt_run,
			bt_entrada,
			bt_saida,
			bt_fecha_notifica,
			rv_notifica,
			lb_notifica,
			cb_perfis,
			bt_ad,
			bt_rm,
		}
	}

	pub fn run(self) {
		let perfis_serializados = match carrega_perfis() {
			Ok(perfis) => perfis,
			Err(e) => {
				println!("Erro ao carregar os perfis: {}", e);
				serializa_yaml(&popula_perfis())
			}
		};

		let perfis: Rc<RefCell<_>> = Rc::new(RefCell::new(desserializa_yaml(perfis_serializados)));

		// Cria o modelo StringList para o DropDown
		let model = StringList::new(&[]);
		inicia_combo(&model, &perfis);
		self.cb_perfis.set_model(Some(&model));

		// Configura a coluna de ID e seleção inicial
		self.cb_perfis.set_selected(0);

		let nome_perfil = match self.cb_perfis.selected_item() {
			Some(item) => {
				if let Some(string_obj) = item.downcast_ref::<gtk::StringObject>() {
					string_obj.string().to_string()
				} else {
					String::new()
				}
			}
			None => String::new(),
		};

		if !nome_perfil.is_empty() {
			atualiza_campos(
				nome_perfil,
				&self.ent_latitude,
				&self.ent_longitude,
				&perfis,
			);
		}

		let window = self.window.clone();
		let uri_entrada: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::new()));
		let uri_saida: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::new()));

		{
			let uri_clone = uri_entrada.clone();

			self.bt_entrada.connect_clicked(move |_| {
				println!("Teste do callback antes de criar filechooser");

				let file_dialog = gtk::FileDialog::new();
				file_dialog.set_title("Open File");

				let window_clone = window.clone();
				let uri_clone2 = uri_clone.clone();

				glib::spawn_future_local(async move {
					match file_dialog.open_future(Some(&window_clone)).await {
						Ok(file) => {
							if let Some(path) = file.path() {
								let mut uri = uri_clone2.borrow_mut();
								*uri = path;
								println!("TESTE 1 {:?}", &uri_clone2);
							}
						}
						Err(e) => eprintln!("Erro ao abrir arquivo: {}", e),
					}
				});
			});
		}

		{
			let uri_clone = uri_saida.clone();
			let window = self.window.clone();
			let ent_planilha_clone = self.ent_planilha.clone();

			self.bt_saida.connect_clicked(move |_| {
				println!("Teste do callback antes de criar filechooser");

				let file_dialog = gtk::FileDialog::new();
				file_dialog.set_title("Escolha o diretório para salvar");

				let window_clone = window.clone();
				let uri_clone2 = uri_clone.clone();
				let ent_planilha_clone2 = ent_planilha_clone.clone();

				glib::spawn_future_local(async move {
					match file_dialog.save_future(Some(&window_clone)).await {
						Ok(file) => {
							if let Some(path) = file.path() {
								let mut uri = uri_clone2.borrow_mut();
								*uri = path.clone();
								ent_planilha_clone2.set_text(path.to_str().unwrap());
								println!("TESTE 1.1 {:?}", &uri_clone2);
							}
						}
						Err(e) => eprintln!("Erro ao salvar arquivo: {}", e),
					}
				});
			});
		}

		{
			// Bloco de execução da busca

			let ent_latitude_clone = self.ent_latitude.clone();
			let ent_longitude_clone = self.ent_longitude.clone();
			let rv_notifica_clone = self.rv_notifica.clone();
			let lb_notifica_clone = self.lb_notifica.clone();
			let uri_entrada_clone = uri_entrada.clone();
			let uri_saida_clone = uri_saida.clone();
			let uri_entrada_clone2 = uri_entrada.clone();
			let uri_saida_clone2 = uri_saida.clone();

			self.bt_run.connect_clicked(move |_| {
                if Dados::check(
                    &*uri_entrada_clone.borrow(),
                    &*uri_saida_clone.borrow(),
                    &ent_latitude_clone,
                    &ent_longitude_clone,
                    &rv_notifica_clone,
                    &lb_notifica_clone,
                ) {
                    let dados = Dados::new(
                        &uri_entrada_clone2,
                        &uri_saida_clone2,
                        &ent_latitude_clone,
                        &ent_longitude_clone,
                    );

                    let texto = fs::read_to_string(&*dados.uri_entrada.borrow());

                    match texto {
                        Ok(_) => {
                            let ret = analisa_texto(
                                &*dados.uri_entrada.borrow(),
                                &dados.uri_saida,
                                dados.latitude,
                                dados.longitude,
                            );
                            match ret {
                                true => {}
                                false => {
                                    lb_notifica_clone
                                        .set_label("Números direfentes de longitudes e latitudes.");
                                    rv_notifica_clone.set_reveal_child(true);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Erro no processamento do texto: {}", e);
                            lb_notifica_clone.set_label(
                                "A codificação do arquivo de entrada deve ser UTF-8!\nConverta-o em um editor de texto.",
                            );
                            rv_notifica_clone.set_reveal_child(true);
                        }
                    }
                } else {
                    println!("Faltam parâmetros!");
                }
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

			// Use a variante _local para não exigir Send + Sync
			combo.connect_notify_local(Some("selected"), move |cb, _pspec| {
				if let Some(obj) = cb.selected_item() {
					if let Ok(string_obj) = obj.downcast::<gtk::StringObject>() {
						let nome_perfil = string_obj.string().to_string();
						atualiza_campos(nome_perfil, &ent_1, &ent_2, &perfis_clone);
					}
				}
			});
		}

		{
			//	let cb_perfis_clone = self.cb_perfis.clone();
			let perfis_clone0 = perfis.clone();
			let window_clone = self.window.clone();
			let model_clone = model.clone();

			self.bt_ad.connect_clicked(move |_| {
				let cadastra = Cadastra::new();
				let cadastra_clone = cadastra.clone();
				let model_clone = model_clone.clone();

				cadastra.dialog.set_transient_for(Some(&window_clone));

				let perfis_clone1 = perfis_clone0.clone();
				cadastra.bt_preencher.connect_clicked(move |_| {
					if &cadastra_clone.ent_dialog_latitude.text().to_string() == ""
						|| &cadastra_clone.ent_dialog_longitude.text().to_string() == ""
						|| &cadastra_clone.ent_dialog_perfil.text().to_string() == ""
					{
					} else {
						let nome_perfil = &cadastra_clone.ent_dialog_perfil.text().to_string();

						adiciona_perfil(
							nome_perfil.to_string(),
							&cadastra_clone.ent_dialog_latitude.text().to_string(),
							&cadastra_clone.ent_dialog_longitude.text().to_string(),
							&perfis_clone1,
						);

						model_clone.append(nome_perfil);
						println!(
							"Teste do botão fecha diálogo
                                \nO nome do perfil dentro do closure do bt-fecha é: {}
                                \nA expressão da latitude é {}
                                \nA expressão da longitude é {}
                                \no conteúdo dos perfiles no closure é: {:?}",
							nome_perfil,
							&cadastra_clone.ent_dialog_latitude.text().to_string(),
							&cadastra_clone.ent_dialog_longitude.text().to_string(),
							&perfis_clone1
						);
						cadastra_clone.dialog.close();
					}
				});
				cadastra.dialog.set_visible(true);
			});
		}

		{
			let perfis_clone = perfis.clone();
			self.window.connect_close_request(move |_| {
				let map = perfis_clone.borrow();
				match salva_perfis(serializa_yaml(&map)) {
					Ok(_) => glib::Propagation::Proceed,
					Err(e) => {
						println!("Erro ao salvar os perfis: {}", e);
						Propagation::Stop
					}
				}
			});
		}

		{
			let perfis_clone = perfis.clone();
			let win = self.window.clone();
			self.bt_fechar.connect_clicked(move |_| {
				let map = perfis_clone.borrow();
				match salva_perfis(serializa_yaml(&map)) {
					Ok(_) => (),
					Err(e) => println!("Erro ao salvar os perfis: {}", e),
				};
				win.destroy();
			});
		}

		{
			let perfis_clone = perfis.clone();
			let model_clone = model.clone();
			let cb_perfis_clone = self.cb_perfis.clone();

			self.bt_rm.connect_clicked(move |_| {
				if let Some(selected_item) = cb_perfis_clone.selected_item() {
					if let Some(string_obj) = selected_item.downcast_ref::<gtk::StringObject>() {
						let perfil_ativo = string_obj.string().to_string();
						remove_perfil(perfil_ativo, &perfis_clone);
						atualiza_combo(&model_clone, &perfis_clone);
					}
				}
			});
		}

		self.window.set_visible(true);
	}
}

pub fn inicia_combo(model: &StringList, perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>) {
	let map = perfis.borrow();
	for key in map.keys() {
		model.append(key);
	}
}

pub fn atualiza_campos(
	nome_perfil: String,
	ent_latitude: &Entry,
	ent_longitude: &Entry,
	perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>,
) {
	set_entrys(ent_latitude, ent_longitude, &nome_perfil, perfis);
}

pub fn atualiza_combo(model: &StringList, perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>) {
	// Remove todos os itens existentes
	let n_items = model.n_items();
	for i in (0..n_items).rev() {
		model.remove(i);
	}

	// Adiciona todos os itens do mapa
	let map = perfis.borrow();
	for key in map.keys() {
		model.append(key);
	}
}

pub fn set_entrys(
	entry_latitude: &Entry,
	entry_longitude: &Entry,
	nome_perfil: &String,
	perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>,
) {
	let map = perfis.borrow();

	if let Some(expressoes) = map.get(nome_perfil) {
		entry_latitude.set_text(&expressoes.latitude);
		entry_longitude.set_text(&expressoes.longitude);
	}
}

pub fn adiciona_perfil(
	perfil_n: String,
	latitude_n: &String,
	longitude_n: &String,
	perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>,
) {
	let expressoes = Expressoes {
		latitude: latitude_n.to_string(),
		longitude: longitude_n.to_string(),
	};

	let mut map: RefMut<_> = perfis.borrow_mut();
	map.insert(perfil_n, expressoes);
}

pub fn remove_perfil(perfil_n: String, perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>) {
	let mut map: RefMut<_> = perfis.borrow_mut();
	map.remove(&perfil_n);
}
