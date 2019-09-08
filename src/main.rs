#![windows_subsystem = "windows"]
#[allow(unused)]
extern crate gtk;
extern crate regex;

use gtk::*;
use regex::Regex;
use std::fs;
use std::string::String;
use csv::*;

//use std::path::Path;
use std::path::PathBuf;
//use std::ffi::OsStr;

pub struct MainWindow {
	pub glade: Builder,
	pub window: Window,
	pub ent_express1: Entry,
	pub ent_express2: Entry,
	pub bt_fechar: Button,
	pub bt_run: Button,
	pub bt_fs1: FileChooserButton,
	pub bt_fs2: FileChooserButton,
	pub bt_close: Button,
	pub revealer: Revealer,
	pub label2: Label,
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
		let bt_fs1: FileChooserButton = glade.get_object("bt_fs1").unwrap();
		let bt_fs2: FileChooserButton = glade.get_object("bt_fs2").unwrap();
		let bt_close: Button = glade.get_object("button2").unwrap();
		let revealer: Revealer = glade.get_object("revealer2").unwrap();
		let label2: Label = glade.get_object("label2").unwrap();

		window.connect_delete_event(move |_,_| {
			main_quit();
			Inhibit(false)
		});

		bt_fechar.connect_clicked(move |_| {
			main_quit();
			Inhibit(false);
		});

		let btfs1 = bt_fs1.clone();
		let btfs2 = bt_fs2.clone();
		let ent_express1_clone = ent_express1.clone();
		let ent_express2_clone = ent_express2.clone();
		let ent_saida_clone = ent_saida.clone();
		let revealer_clone = revealer.clone();
		let label2_clone = label2.clone();

		bt_run.connect_clicked(move |_| {

			if Dados::check(&btfs1, &btfs2, &ent_express1_clone, &ent_express2_clone, &ent_saida_clone, &revealer_clone, &label2_clone) {

			 	let dados = Dados::new(&btfs1, &btfs2, &ent_express1_clone, &ent_express2_clone, &ent_saida_clone);

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
			 			label2_clone.set_label("A codificação do arquivo de entrada deve ser UTF-8! Converta-o em um editor de texto.");
			 			revealer_clone.set_reveal_child(true);
			 		},
			 	}
			} else { println!("Faltam parâmetros!"); }
		});

		let revealer_clone2 = revealer.clone();

		bt_close.connect_clicked(move |_| {
			revealer_clone2.set_reveal_child(false);
		});

		MainWindow { glade, window, ent_express1, ent_express2, bt_fechar, bt_run, bt_fs1, bt_fs2, bt_close, revealer, label2 }
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

fn analisa_memorial (uri1: PathBuf, uri2: PathBuf, expressao1: String,  expressao2: String) -> Result<()> {
	let texto = fs::read_to_string(uri1)?; // Tirei o método .unwrap() e coloquei o operador ?
	let text = &String::from(texto);

	const VEC_SIZE: usize = 13;
	let mut vetor1 = Vec::with_capacity(VEC_SIZE);
	let mut vetor2 = Vec::with_capacity(VEC_SIZE);
	// r"\d.\d{3}.\d{3},\d{3}" -> quando utilizado através do gtk_entry não foi
	// necessário o caracteres r e as aspas, apenas
	// a expressão regular propriamente dita.
    for mat2 in Regex::new(&expressao1).unwrap().find_iter(text) {
    	let start = mat2.start() as usize;
    	let end = mat2.end() as usize;
    	vetor1.push(&text[start..end]);
    }

	for mat2 in Regex::new(&expressao2).unwrap().find_iter(text) {
    	let start = mat2.start() as usize;
    	let end = mat2.end() as usize;
    	vetor2.push(&text[start..end]);
    }
    gera_csv (vetor1, vetor2, uri2).expect("Não foi possível utilizar a uri informada pela função Dados::new()");
    Ok(())
}

fn gera_csv (vec: Vec<&str>,vec1: Vec<&str>, uri2: PathBuf) -> Result<()> {
	let mut wtr = Writer::from_path(uri2)?;
	wtr.write_record(&["N","E"])?;

	for i in 0..vec.len() {
		wtr.write_record(&[vec[i],vec1[i]]).expect("Não foi possível gravar os dados do vetor");
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
	fn new (btfs1:		&FileChooserButton,
			btfs2:		&FileChooserButton,
			ent_exp1:	&Entry,
			ent_exp2:	&Entry,
			ent_nome:	&Entry)-> Dados {

		let uri_entrada = btfs1.get_filename().unwrap();
		println!("URI da entrada {:?}\n", uri_entrada);

		let mut uri_saida = btfs2.get_filename().unwrap();

		println!("URI saída é: {:?}", btfs2.get_filename().unwrap());

		let expressao_1: String = ent_exp1.get_text().unwrap().to_string();
		let expressao_2: String = ent_exp2.get_text().unwrap().to_string();
		let nome_csv: String = ent_nome.get_text().unwrap().to_string();
		// uri_saida.push_str(&"/".to_string());
		uri_saida.push(&nome_csv);
		uri_saida.set_extension("csv");
		//uri_saida.push_str(&nome_csv);
	Dados { uri_entrada, uri_saida, expressao_1, expressao_2, nome_csv }
	}

	fn check (btfs1:		&FileChooserButton,
			btfs2:		&FileChooserButton,
			ent_exp1:	&Entry,
			ent_exp2:	&Entry,
			ent_nome:	&Entry,
			revealer:	&Revealer,
			label: 		&Label) -> bool {

			let mut resultado: bool = false;

			if ent_nome.get_text_length() != 0 {
				if ent_exp1.get_text_length() != 0 {
					if ent_exp2.get_text_length() != 0 {
						if let Some(teste) = btfs1.get_uri() {
						 	println!("O widget bt_fs1 tem: {:?}", teste.as_str());
						 	if let Some(teste) = btfs2.get_uri() {
						 		println!("O widget bt_fs1 tem: {:?}", teste.as_str());
						 		resultado = true;
						 	} else {
								label.set_label("Selecione o diretório de destino!");
								revealer.set_reveal_child(true);
					 		}

						} else {
							label.set_label("Selecione o arquivo a ser analisado!");
							revealer.set_reveal_child(true);
						};

					} else {
						label.set_label("Informe a segunda expressão regular!");
						revealer.set_reveal_child(true);
					}

				} else {
					label.set_label("Informe a primeira expressão regular!");
					revealer.set_reveal_child(true);
				}
			} else {
				label.set_label("Informe o nome da planilha resultante!");
				revealer.set_reveal_child(true);
			}

		resultado
	}
}
#[allow(unused)]
fn check_utf8 (uri1: PathBuf) {

}
