extern crate gtk;

use gtk::*;
use std::path::PathBuf;

pub struct Dados {
	pub uri_entrada:	PathBuf,
	pub uri_saida:		PathBuf,
	pub latitude:		String,
	pub longitude:		String,
	pub nome_csv:		String,
}

impl Dados {
	pub fn new (bt_entrada:		&FileChooserButton,
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

	pub fn check(	bt_entrada:		&FileChooserButton,
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


