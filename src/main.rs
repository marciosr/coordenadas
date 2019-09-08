#![windows_subsystem = "windows"]
#[allow(unused)]
extern crate gtk;
extern crate regex;

use gtk::*;
use regex::Regex;
use std::fs;
use std::string::String;
use csv::*;
use std::path::PathBuf;

pub struct MainWindow {
	pub glade: Builder,
	pub window: Window,
	pub ent_express1: Entry,
	pub ent_express2: Entry,
	pub bt_fechar: Button,
	pub bt_run: Button,
	pub bt_entrada: FileChooserButton,
	pub bt_saida: FileChooserButton,
	pub bt_fecha_notifica: Button,
	pub rv_notifica: Revealer,
	pub lb_notifica: Label,
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let glade_src = include_str!("ui.glade");
		let glade = gtk::Builder::new_from_string(glade_src);
		let window: gtk::Window = glade.get_object("window").unwrap();
		let ent_express1: Entry = glade.get_object("ent_express1").unwrap();
		let ent_express2: Entry = glade.get_object("ent_express2").unwrap();
		let ent_saida: Entry = glade.get_object("ent_saida").unwrap();
		let bt_fechar: Button = glade.get_object("bt_fechar").unwrap();
		let bt_run: Button = glade.get_object("bt_run").unwrap();
		let bt_entrada: FileChooserButton = glade.get_object("bt_entrada").unwrap();
		let bt_saida: FileChooserButton = glade.get_object("bt_saida").unwrap();
		let bt_fecha_notifica: Button = glade.get_object("bt_fecha_notifica").unwrap();
		let rv_notifica: Revealer = glade.get_object("rv_notifica").unwrap();
		let lb_notifica: Label = glade.get_object("label2").unwrap();

		window.connect_delete_event(move |_,_| {
			main_quit();
			Inhibit(false)
		});

		bt_fechar.connect_clicked(move |_| {
			main_quit();
			Inhibit(false);
		});

		let bt_entrada_clone = bt_entrada.clone();
		let bt_saida_clone = bt_saida.clone();
		let ent_express1_clone = ent_express1.clone();
		let ent_express2_clone = ent_express2.clone();
		let ent_saida_clone = ent_saida.clone();
		let rv_notifica_clone = rv_notifica.clone();
		let lb_notifica_clone = lb_notifica.clone();

		bt_run.connect_clicked(move |_| {

			if Dados::check(&bt_entrada_clone, &bt_saida_clone, &ent_express1_clone, &ent_express2_clone, &ent_saida_clone, &rv_notifica_clone, &lb_notifica_clone) {

			 	let dados = Dados::new(&bt_entrada_clone, &bt_saida_clone, &ent_express1_clone, &ent_express2_clone, &ent_saida_clone);
			 	let texto = fs::read_to_string(&dados.uri_entrada);

			 	match texto {
			 		Ok(_)	=> {
					 	analisa_memorial (	dados.uri_entrada,
					 						dados.uri_saida,
					 						dados.expressao_1,
					 						dados.expressao_2).expect("Não foi possível carregar o arquivo de texto");
			 		},
			 		Err(e)		=> {
			 			println!("Erro no processamento do texto: {}", e);
			 			lb_notifica_clone.set_label("A codificação do arquivo de entrada deve ser UTF-8! Converta-o em um editor de texto.");
			 			rv_notifica_clone.set_reveal_child(true);
			 		},
			 	}
			} else { println!("Faltam parâmetros!"); }
		});

		let rv_notifica_clone2 = rv_notifica.clone();
		bt_fecha_notifica.connect_clicked(move |_| {
			rv_notifica_clone2.set_reveal_child(false);
		});

		MainWindow { glade, window, ent_express1, ent_express2, bt_fechar, bt_run, bt_entrada, bt_saida, bt_fecha_notifica, rv_notifica, lb_notifica }
	}
}


fn main() {
	if gtk::init().is_err() {
    	println!("A inicialização do gtk falhou.");
    	return;
	}

	let app = MainWindow::new();
	app.window.show_all();

    gtk::main();
}

fn analisa_memorial (uri_entrada: PathBuf, uri_saida: PathBuf, expressao_n: String,  expressao_e: String) -> Result<()> {
	let texto = fs::read_to_string(uri_entrada)?; // Tirei o método .unwrap() e coloquei o operador ?
	let text = &String::from(texto);

	const VEC_SIZE: usize = 13;
	let mut vetor1 = Vec::with_capacity(VEC_SIZE);
	let mut vetor2 = Vec::with_capacity(VEC_SIZE);
	// r"\d.\d{3}.\d{3},\d{3}" -> quando utilizado através do gtk_entry não foi
	// necessário o caracteres r e as aspas, apenas
	// a expressão regular propriamente dita.
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
	wtr.write_record(&["N","E"])?;

	for i in 0..vec.len() {
		wtr.write_record(&[vec[i].as_str(),vec1[i].as_str()]).expect("Não foi possível gravar os dados do vetor");
	}

	wtr.flush()?;
	Ok(())
}

#[allow(dead_code)]

struct Dados {
	pub uri_entrada:	PathBuf,
	pub uri_saida:		PathBuf,
	pub expressao_1:	String,
	pub expressao_2:	String,
	pub nome_csv:		String,
}


#[allow(dead_code)]
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

		let expressao_1: String = ent_exp1.get_text().unwrap().to_string();
		let expressao_2: String = ent_exp2.get_text().unwrap().to_string();
		let nome_csv: String = ent_nome.get_text().unwrap().to_string();

		uri_saida.push(&nome_csv);
		uri_saida.set_extension("csv");

		Dados { uri_entrada, uri_saida, expressao_1, expressao_2, nome_csv }
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
