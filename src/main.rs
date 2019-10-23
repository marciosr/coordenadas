#![windows_subsystem = "windows"]
extern crate gtk;
extern crate regex;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::string::String;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;

use gtk::*;
use regex::Regex;
use csv::*;
use serde::{Serialize, Deserialize};

pub struct MainWindow {
	pub glade:				Builder,
	pub window:				Window,
	pub ent_latitude:		Entry,
	pub ent_longitude:		Entry,
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

		inicia_combo (&cb_perfis, &perfis);

		cb_perfis.set_id_column(1); // Garante que haja um perfil ativo, assim não havera o crash de unwrap() on None.
		cb_perfis.set_active(Some(0));

		let perfis = match cb_perfis.get_active_text() {
			Some(_ativo) => perfis,
			None =>	{
				let perfis_populados: Rc<RefCell<_>> = Rc::new(RefCell::new(popula_perfis()));
				inicia_combo (&cb_perfis, &perfis_populados);
				perfis_populados
			},
		};

		cb_perfis.set_active(Some(0));

		let nome_perfil = cb_perfis.get_active_text().unwrap(); // Possível problema de unwrap sobre None
		atualiza_campos(nome_perfil.to_string(), &ent_latitude, &ent_longitude, &perfis);

		{ // Bloco de execussão da busca
			let bt_entrada_clone = bt_entrada.clone();
			let bt_saida_clone = bt_saida.clone();
			let ent_latitude_clone = ent_latitude.clone();
			let ent_longitude_clone = ent_longitude.clone();
			let ent_saida_clone = ent_saida.clone();
			let rv_notifica_clone = rv_notifica.clone();

			let lb_notifica_clone = lb_notifica.clone();

			bt_run.connect_clicked(move |_| {

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
			let rv_notifica_clone2 = rv_notifica.clone();
			bt_fecha_notifica.connect_clicked(move |_| {
				rv_notifica_clone2.set_reveal_child(false);
			});
		}

		{
			let combo = cb_perfis.clone();
			let ent_1 = ent_latitude.clone();
			let ent_2 = ent_longitude.clone();
			let perfis_clone = perfis.clone();

			combo.connect_changed(move |cb| {
				println!("Linha 114: O id do combo é antes da mudança: {:?}", cb.get_active_id());
				match cb.get_active_text() {
					Some(_texto) => {
						let nome_perfil = cb.get_active_text().unwrap();
						atualiza_campos(nome_perfil.to_string(), &ent_1, &ent_2, &perfis_clone);
						println!("Linha 114: nome do perfil é: {}", nome_perfil);
						println!("Linha 114: O id do combo é: {:?}", cb.get_active_id());
					},
					None => println!("Não há texto ativo"),

				}


			});
		}

		{
			let cb_perfis_clone = cb_perfis.clone();
			let perfis_clone0 = perfis.clone();
			let window_clone = window.clone();
			bt_ad.connect_clicked(move |_| {

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

		{
			// let perfis_clone = perfis.clone();
			// window.connect_delete_event(move |_,_| {
			// 	let map = perfis_clone.borrow();
			// 	match salva_perfis(serializa_yaml(&map)) {
			// 		Ok(a) => a,
			// 		Err(e) => println!("Erro ao salvar os perfis: {}", e),
			// 	};
			// 	main_quit();
			// 	Inhibit(false)
			// });
		}

		{
			let perfis_clone = perfis.clone();

			bt_fechar.connect_clicked(move |_| {
				let map = perfis_clone.borrow();
				println!("variável pefris para a função salvar {:?}", perfis_clone);
				println!("variável map para a função salvar {:?}", map);
				match salva_perfis(serializa_yaml(&map)) {
					Ok(a) => a,
					Err(e) => println!("Erro ao salvar os perfis: {}", e),
				};
				main_quit();
				//Inhibit(false);
			});
		}

		{
			let combo = cb_perfis.clone();
			let perfis_clone = perfis.clone();

			bt_rm.connect_clicked(move|_| {
				remove_perfil(&combo, &perfis_clone);
			});
		}

		MainWindow {
	        glade,
	        window,
	        ent_latitude,
	        ent_longitude,
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
}

pub struct Cadastra {
	pub dialog:					Window,
	pub ent_dialog_perfil:		Entry,
	pub ent_dialog_latitude:	Entry,
	pub ent_dialog_longitude:	Entry,
	pub bt_fecha_dialogo:		Button,
	pub bt_preencher:			Button
}

impl Cadastra {
	fn new() -> Rc<Self> {
		let glade_src = include_str!("dialog.glade");
		let glade = gtk::Builder::new_from_string(glade_src);
		let dialog: gtk::Window = glade.get_object("dialog").unwrap();

		let ent_dialog_perfil: Entry = glade.get_object("ent_dialog_perfil").unwrap();
		let ent_dialog_latitude: Entry = glade.get_object("ent_dialog_latitude").unwrap();
		let ent_dialog_longitude: Entry = glade.get_object("ent_dialog_longitude").unwrap();
		let bt_fecha_dialogo: Button = glade.get_object("bt_fecha_dialogo").unwrap();
		let bt_preencher: Button = glade.get_object("bt_preencher").unwrap();

		{
			let dialog_clone = dialog.clone();
			bt_fecha_dialogo.connect_clicked (move |_| {
				dialog_clone.destroy();
			});
		}

		let cadastra = Rc::new(Self {
			dialog,
			ent_dialog_perfil,
			ent_dialog_latitude,
			ent_dialog_longitude,
			bt_fecha_dialogo,
			bt_preencher
		});
		cadastra
	}
}

fn main() {
	if gtk::init().is_err() {
    	println!("A inicialização do gtk falhou.");
    	return;
	}

	let app = MainWindow::new();
	app.window.show();

    gtk::main();
}

fn analisa_texto (uri_entrada: PathBuf, uri_saida: PathBuf, expressao_n: String,  expressao_e: String) -> Result<()> {
	let texto = fs::read_to_string(uri_entrada)?; // Tirei o método .unwrap() e coloquei o operador ?
	let text = &String::from(texto);

	const VEC_SIZE: usize = 13;
	let mut vetor1 = Vec::with_capacity(VEC_SIZE);
	let mut vetor2 = Vec::with_capacity(VEC_SIZE);
	// r"\d.\d{3}.\d{3},\d{3}" -> quando utilizado através do gtk_entry não foi
	// necessário o caracteres r e as aspas, apenas
	// a expressão regular propriamente dita.
	println!("Expressão latitude: {}", expressao_n);
	println!("Expressão longitude: {}", expressao_e);
    for correspondencia in Regex::new(&expressao_n).unwrap().find_iter(text) {
    	let start = correspondencia.start() as usize;
    	let end = correspondencia.end() as usize;

    	vetor1.push(&text[start..end]);
    }

	for correspondencia in Regex::new(&expressao_e).unwrap().find_iter(text) {
    	let start = correspondencia.start() as usize;
    	let end = correspondencia.end() as usize;

    	vetor2.push(&text[start..end]);
    }

	let mut vetor3 = Vec::with_capacity(VEC_SIZE);
    for x in vetor1.iter_mut() {
    	vetor3.push(x.replace(".",""));
    }

    let mut vetor4 = Vec::with_capacity(VEC_SIZE);
   	for x in vetor2.iter_mut() {
    	vetor4.push(x.replace(".",""));
    }

    gera_csv (vetor3, vetor4, uri_saida).expect("Não foi possível utilizar a uri informada pela função Dados::new()");
    Ok(())
}

fn gera_csv (vec: Vec<String>,vec1: Vec<String>, uri2: PathBuf) -> Result<()> {
	let mut wtr = Writer::from_path(uri2)?;
	wtr.write_record(&["Latitude","Longitude"])?;

	for i in 0..vec.len() {
		wtr.write_record(&[vec[i].as_str(),vec1[i].as_str()]).expect("Não foi possível gravar os dados do vetor");
	}

	wtr.flush()?;
	Ok(())
}

struct Dados {
	pub uri_entrada:	PathBuf,
	pub uri_saida:		PathBuf,
	pub latitude:		String,
	pub longitude:		String,
	pub nome_csv:		String,
}

impl Dados {
	fn new (bt_entrada:		&FileChooserButton,
			bt_saida:		&FileChooserButton,
			ent_exp1:		&Entry,
			ent_exp2:		&Entry,
			ent_nome:		&Entry)-> Dados {

		let uri_entrada = bt_entrada.get_filename().unwrap();
		println!("URI da entrada {:?}\n", uri_entrada);

		let mut uri_saida = bt_saida.get_filename().unwrap();
		println!("URI saída é: {:?}", bt_saida.get_filename().unwrap());

		let latitude: String = ent_exp1.get_text().unwrap().to_string();
		let longitude: String = ent_exp2.get_text().unwrap().to_string();
		let nome_csv: String = ent_nome.get_text().unwrap().to_string();

		uri_saida.push(&nome_csv);
		uri_saida.set_extension("csv");

		Dados { uri_entrada, uri_saida, latitude, longitude, nome_csv }
	}

	fn check(	bt_entrada:		&FileChooserButton,
				bt_saida:		&FileChooserButton,
				ent_exp1:		&Entry,
				ent_exp2:		&Entry,
				ent_nome:		&Entry,
				rv_notifica:	&Revealer,
				lb_notifica: 	&Label) -> bool {

			let mut resultado: bool = false;

			if ent_nome.get_text_length() != 0 {
				if ent_exp1.get_text_length() != 0 {
					if ent_exp2.get_text_length() != 0 {
						if let Some(teste) = bt_entrada.get_uri() {
						 	println!("O widget bt_entrada tem: {:?}", teste.as_str());
						 	if let Some(teste) = bt_saida.get_uri() {
						 		println!("O widget bt_entrada tem: {:?}", teste.as_str());
						 		resultado = true;
						 	} else {
								lb_notifica.set_label("Selecione o diretório de destino!");
								rv_notifica.set_reveal_child(true);
					 		}
						} else {
							lb_notifica.set_label("Selecione o arquivo a ser analisado!");
							rv_notifica.set_reveal_child(true);
						};

					} else {
						lb_notifica.set_label("Informe a segunda expressão regular!");
						rv_notifica.set_reveal_child(true);
					}

				} else {
					lb_notifica.set_label("Informe a primeira expressão regular!");
					rv_notifica.set_reveal_child(true);
				}
			} else {
				lb_notifica.set_label("Informe o nome da planilha resultante!");
				rv_notifica.set_reveal_child(true);
			}

		resultado
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Expressoes {
	latitude: String,
	longitude: String
}

fn set_entrys (	entry_latitude: &Entry,
				entry_longitude: &Entry,
				nome_perfil: &String,
				perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {
	let map = perfis.borrow();

	let expressoes = map.get(nome_perfil).unwrap();
	entry_latitude.set_text(&expressoes.latitude);
	entry_longitude.set_text(&expressoes.longitude);
}

fn adiciona_perfil (	perfil_n: String,
						latitude_n: &String,
						longitude_n: &String,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	let expressoes = Expressoes { latitude: latitude_n.to_string(), longitude: longitude_n.to_string()};

	let mut map: RefMut<_> = perfis.borrow_mut();
	println!("O conteúdo dos perfils dentro da função adiciona perfi é: {:?}", map); // Para testes!
	map.insert(perfil_n, expressoes);
}

fn salva_perfis (serializado: String) -> std::io::Result<()> {
	let mut file = File::create("perfis.yaml")?;
	file.write_all(&serializado.as_bytes())?;

	Ok(())
}

fn carrega_perfis () -> std::io::Result<(String)> {
	let file = fs::read_to_string("perfis.yaml")?;
	Ok(file)
}

#[allow(dead_code)]
fn serializa (map: &BTreeMap<String, Expressoes>) -> String {
	let serializado = serde_json::to_string(&map).unwrap();
	serializado
}

fn serializa_yaml (map: &BTreeMap<String, Expressoes>) -> String {
	let serializado = serde_yaml::to_string(&map).unwrap();
	serializado
}

#[allow(dead_code)]
fn desserializa (serializado: String) -> BTreeMap<String, Expressoes> {
	let desserializado: BTreeMap<String, Expressoes> = serde_json::from_str(&serializado).unwrap();
	desserializado
}

fn desserializa_yaml (serializado: String) -> BTreeMap<String, Expressoes> {
	let desserializado: BTreeMap<String, Expressoes> = serde_yaml::from_str(&serializado).unwrap();
	desserializado
}

fn inicia_combo (	combo: &ComboBoxText,
					perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>
				) {
	let map = perfis.borrow();
	for (key, _value) in map.iter() {
			println!("Inciando... {:?}", key);
		combo.append_text(&key);
	}
}

fn atualiza_campos (	nome_perfil: String,
						ent_latitude: &Entry,
						ent_longitude: &Entry,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	set_entrys(&ent_latitude, &ent_longitude, &String::from(nome_perfil), perfis);
}

fn popula_perfis () -> BTreeMap<String, Expressoes> {
	let utm = Expressoes { latitude: String::from(r"\d.\d{3}.\d{3},\d{1,3}"), longitude: String::from(r" \d{3}.\d{3},\d{1,3}")};
	let decimal = Expressoes { latitude: String::from(r"[+-]?[3-4]\d\.\d{6}"), longitude: String::from(r"[+-]?[0-2]\d\.\d{6}")};
	let gms = Expressoes { latitude: String::from(r"[0-2]\dS\s[0-5]\d'\s[0-5]\d"), longitude: String::from(r"[3-7]\dW\s[0-5]\d'\s[0-5]\d")};

	let mut perfis = BTreeMap::new();

	perfis.insert("UTM".to_string(), utm);
	perfis.insert("Decimal".to_string(), decimal);
	perfis.insert("Graus, minutos e segundos".to_string(), gms);
	perfis
}

fn remove_perfil (	combo: &ComboBoxText,
					perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	match combo.get_active_text() {
			Some(_ativo) => {

				let perfil = combo.get_active_text().unwrap();

				let mut map: RefMut<_> = perfis.borrow_mut();

				map.remove(perfil.as_str());
				combo.remove_all();

				for (key, _value) in map.iter() {
						println!("{:?}", key);
					combo.append_text(&key);
				}
			},
			None =>	{},
	}
}
