extern crate gtk;

use std::rc::Rc;
use std::cell::RefCell;
use gtk::*;
use gtk::Entry;
use std::path::PathBuf;
use gtk::prelude::EntryExt;
use gtk::prelude::*;

pub struct Dados<'a> {
	pub uri_entrada:	&'a Rc<RefCell<PathBuf>>,
	pub uri_saida:		PathBuf,
	pub latitude:		String,
	pub longitude:		String,
}

impl <'a> Dados <'a> {
		pub fn new (uri_entrada:	&'a Rc<RefCell<PathBuf>>,
					uri_saidaa:		&Rc<RefCell<PathBuf>>,
					ent_exp1:		&Entry,
					ent_exp2:		&Entry )-> Dados<'a> {

		println!("URI da entrada {:?}\n", uri_saidaa);

		let latitude: String = ent_exp1.text().to_string();
		let longitude: String = ent_exp2.text().to_string();

		let tmp = uri_saidaa.borrow();

		let mut uri_saida: PathBuf = PathBuf::new();
		uri_saida.push(&*tmp);
		uri_saida.set_extension("csv");

		Dados { uri_entrada, uri_saida, latitude, longitude }
	}

	pub fn check(	uri_entrada:	&PathBuf,
					uri_saida:		&PathBuf,
					ent_exp1:		&Entry,
					ent_exp2:		&Entry,
					rv_notifica:	&Revealer,
					lb_notifica: 	&Label ) -> bool {

			let mut resultado: bool = false;

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
		resultado
	}
}
