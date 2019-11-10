extern crate gtk;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::*;

use crate::dialogo_cadastra_perfis::Cadastra;
use crate::frontend_data_check::Dados;
use crate::backend::*;


pub struct MainWindow {
	pub glade:				Builder,
	pub window:				Window,
	pub ent_latitude:		Entry,
	pub ent_longitude:		Entry,
	pub ent_saida:			Entry,
	pub bt_fechar:			Button,
	pub bt_run:				Button,
	pub bt_entrada:			FileChooserButton,
	pub bt_saida:			FileChooserButton,
	pub bt_fecha_notifica:	Button,
	pub rv_notifica:		Revealer,
	pub lb_notifica:		Label,
	pub cb_perfis:			ComboBoxText,
	pub bt_ad:				Button,
	pub bt_rm:				Button
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let glade_src = include_str!("main_window.glade");
		let glade = gtk::Builder::new_from_string(glade_src);
		let window: gtk::Window = glade.get_object("window").unwrap();
		let ent_latitude: Entry = glade.get_object("ent_latitude").unwrap();
		let ent_longitude: Entry = glade.get_object("ent_longitude").unwrap();
		let ent_saida: Entry = glade.get_object("ent_saida").unwrap();
		let bt_fechar: Button = glade.get_object("bt_fechar").unwrap();
		let bt_run: Button = glade.get_object("bt_run").unwrap();
		let bt_entrada: FileChooserButton = glade.get_object("bt_entrada").unwrap();
		let bt_saida: FileChooserButton = glade.get_object("bt_saida").unwrap();
		let bt_fecha_notifica: Button = glade.get_object("bt_fecha_notifica").unwrap();
		let rv_notifica: Revealer = glade.get_object("rv_notifica").unwrap();
		let lb_notifica: Label = glade.get_object("label2").unwrap();
		let cb_perfis: ComboBoxText = glade.get_object("cb_perfis").unwrap();
		let bt_ad: Button = glade.get_object("bt_ad").unwrap();
		let bt_rm: Button = glade.get_object("bt_rm").unwrap();

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

		let perfis = match self.cb_perfis.get_active_text() {
			Some(_ativo) => perfis,
			None =>	{
				let perfis_populados: Rc<RefCell<_>> = Rc::new(RefCell::new(popula_perfis()));
				inicia_combo (&self.cb_perfis, &perfis_populados);
				perfis_populados
			},
		};

		self.cb_perfis.set_active(Some(0));

		let nome_perfil = self.cb_perfis.get_active_text().unwrap(); // Possível problema de unwrap sobre None
		atualiza_campos(nome_perfil.to_string(), &self.ent_latitude, &self.ent_longitude, &perfis);

		{ // Bloco de execussão da busca
			let bt_entrada_clone = self.bt_entrada.clone();
			let bt_saida_clone = self.bt_saida.clone();
			let ent_latitude_clone = self.ent_latitude.clone();
			let ent_longitude_clone = self.ent_longitude.clone();
			let ent_saida_clone = self.ent_saida.clone();
			let rv_notifica_clone = self.rv_notifica.clone();

			let lb_notifica_clone = self.lb_notifica.clone();

			self.bt_run.connect_clicked(move |_| {

				if Dados::check(&bt_entrada_clone, &bt_saida_clone, &ent_latitude_clone, &ent_longitude_clone, &ent_saida_clone, &rv_notifica_clone, &lb_notifica_clone) {

				 	let dados = Dados::new(&bt_entrada_clone, &bt_saida_clone, &ent_latitude_clone, &ent_longitude_clone, &ent_saida_clone);
				 	let texto = fs::read_to_string(&dados.uri_entrada);

				 	match texto {
				 		Ok(_)	=> {
						 	analisa_texto (	dados.uri_entrada,
						 					dados.uri_saida,
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
				match cb.get_active_text() {
					Some(_texto) => {
						let nome_perfil = cb.get_active_text().unwrap();
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

					if	&cadastra_clone.ent_dialog_latitude.get_text().unwrap().to_string() == "" ||
						&cadastra_clone.ent_dialog_longitude.get_text().unwrap().to_string() == "" ||
						&cadastra_clone.ent_dialog_perfil.get_text().unwrap().to_string() == "" {
						} else {

							let nome_perfil =  &cadastra_clone.ent_dialog_perfil
																.get_text().unwrap().to_string();

							adiciona_perfil (	nome_perfil.to_string(),
												&cadastra_clone.ent_dialog_latitude
																.get_text().unwrap().to_string(),
												&cadastra_clone.ent_dialog_longitude
																.get_text().unwrap().to_string(),
												&perfis_clone1
											);
							cb_perfis_clone2.append_text(nome_perfil);
							println!("Teste do botão fecha diálogo\nO nome do perfil dentro do closure do bt-fecha é: {}\n
								A expressão da latitude é {}\n
								A expressão da longitude é {}\n
								o conteúdo dos perfiles no closure é: {:?}",
								nome_perfil,
								&cadastra_clone.ent_dialog_latitude
																.get_text().unwrap().to_string(),
								&cadastra_clone.ent_dialog_longitude
																.get_text().unwrap().to_string(),
								&perfis_clone1
								);
							// Por padrão ao adicionar um perfil, este passa a ser o ativo
							// atualiza_campos (&nome_perfil.to_string(), ent_latitude_clone2, ent_longitude_clone2);
							cadastra_clone.dialog.destroy();
					}
				});

				cadastra.dialog.show();

			});
		}

		{	// Para fechar a janela no gtk4, tive que substituir o
			// sinal connect_delete_event por connect_destroy
			let perfis_clone = perfis.clone();
			self.window.connect_destroy(move |_| {
				let map = perfis_clone.borrow();
				match salva_perfis(serializa_yaml(&map)) {
					Ok(a) => a,
					Err(e) => println!("Erro ao salvar os perfis: {}", e),
				};
				main_quit();
			});
		}

		{
			let perfis_clone = perfis.clone();

			self.bt_fechar.connect_clicked(move |_| {
				let map = perfis_clone.borrow();
				match salva_perfis(serializa_yaml(&map)) {
					Ok(a) => a,
					Err(e) => println!("Erro ao salvar os perfis: {}", e),
				};
				main_quit();
			});
		}

		{
			let combo = self.cb_perfis.clone();
			let perfis_clone = perfis.clone();

			self.bt_rm.connect_clicked(move|_| {
				remove_perfil(&combo, &perfis_clone);
			});
		}
	self.window.show();

	}
}
