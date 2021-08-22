extern crate gtk;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use gtk::*;
use gtk::{FileChooser, Entry, EntryBuffer};
use std::path::PathBuf;
use gtk::prelude::{EntryExt, EntryBufferExt};
use gtk::gio::File;
use gtk::prelude::*;


pub struct Dados<'a> {
	pub uri_entrada:	&'a Rc<RefCell<PathBuf>>,
	pub uri_saida:		PathBuf,
	pub latitude:			String,
	pub longitude:		String,
	pub nome_csv:			String,
}

impl <'a> Dados <'a> {
		pub fn new (uri_entrada:		&'a Rc<RefCell<PathBuf>>,
								uri_saidaa:						&Rc<RefCell<PathBuf>>,
								ent_exp1:				&Entry,
								ent_exp2:				&Entry,
								ent_nome:				&Entry)-> Dados<'a> {

		println!("URI da entrada {:?}\n", uri_entrada);
		// let buffer = ent_exp1.buffer();
		// let latitude = buffer.to_string();
		let latitude: String = ent_exp1.text().to_string();
		let longitude: String = ent_exp2.text().to_string();
		let nome_csv: String = ent_nome.text().to_string();

		let tmp = uri_saidaa.borrow();

		let mut uri_saida: PathBuf = PathBuf::new();
		uri_saida.push(&*tmp);
		uri_saida.push(&nome_csv);
		println!("Valor de ent_exp1 {}", &latitude);
		println!("Valor de ent_exp2 {}", &longitude);
		println!("Valor de nome_csv {}", &nome_csv);
		uri_saida.set_extension("csv");

		Dados { uri_entrada, uri_saida, latitude, longitude, nome_csv }
	}

	pub fn check(	uri_entrada:	&PathBuf,
								uri_saida:		&PathBuf,
								ent_exp1:			&Entry,
								ent_exp2:			&Entry,
								ent_nome:			&Entry,
								rv_notifica:	&Revealer,
								lb_notifica: 	&Label) -> bool {

			let mut resultado: bool = false;

			println!("\nTestes dentro da função Dados::check()\n");
			println!("Comprimento do texto em ent_latitude {}", ent_exp1.text_length());
			println!("Texto do gtkentrybuffer {}",ent_exp1.text());

			if ent_nome.text_length() != 0 {
				if ent_exp1.text_length() != 0 {
					if ent_exp2.text_length() != 0 {
						if let Some(teste) = Some(uri_entrada) {
						 	println!("O widget bt_entrada tem: {:?}", teste.to_str());
						 	if let Some(teste) = Some(uri_saida) {
						 		println!("O widget bt_entrada tem: {:?}", teste.to_str());
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


