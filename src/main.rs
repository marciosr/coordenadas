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
use std::rc::Rc;

pub struct MainWindow {
	pub glade:			Builder,
	pub window:			Window,
	pub ent_latitude:	Entry,
	pub ent_longitude:	Entry,
	pub bt_fechar:		Button,
	pub bt_run:			Button,
	pub bt_entrada:		FileChooserButton,
	pub bt_saida:		FileChooserButton,
	pub bt_fecha_notifica: Button,
	pub rv_notifica:	Revealer,
	pub lb_notifica:	Label,
	pub cb_tipo_coord:	ComboBoxText,
	pub bt_teste:		Button,
	pub bt_ad:			Button
	//pub dialog:			Dialog,
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let glade_src = include_str!("ui.glade");
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
		let cb_tipo_coord: ComboBoxText = glade.get_object("cb_tipo_coord").unwrap();
		let bt_teste: Button = glade.get_object("bt_teste").unwrap();
		let bt_ad: Button = glade.get_object("bt_ad").unwrap();

		
		struct Expressoes {
			latitude:	String,
			longitude:	String
		}
		
		let perfis_serializados = carrega_perfis();
		let perfis = desserializa(perfis_serializados);
		
		atualiza_entradas(	&ent_latitude,
							&ent_longitude,
							map,
							nome_perfil );
		 
				
		
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
			let combo = cb_tipo_coord.clone();
			let ent_1 = ent_latitude.clone();
			let ent_2 = ent_longitude.clone();
			
			combo.connect_changed(move |c| {
				let tipo = c.get_active_id().unwrap();
				set_entrys(&ent_1, &ent_2, &String::from(tipo)); 
				//println!("O tipo é: {}", tipo); 
			});
		}
		// let dia_clone = cadastra.clone();
		// let mut personal = ExprePers {pers_latitude:String::from("teste"), pers_longitude:String::from("teste")};
		{
			let ent_latitude_clone = ent_latitude.clone();
			let ent_longitude_clone = ent_longitude.clone();
			let cb_tipo_coord_clone = cb_tipo_coord.clone();
			bt_ad.connect_clicked(move |_| {
				//println!("O conteúdo do dialog é: {:?}", dia_clone.dialog);
				//dia_clone.dialog.run();
				//d.show_all();
				let cadastra = Cadastra::new();
				let cadastra_clone = cadastra.clone();
				let cb_tipo_coord_clone2 = cb_tipo_coord_clone.clone();
				let ent_latitude_clone2 = ent_latitude_clone.clone();
				let ent_longitude_clone2 = ent_longitude_clone.clone();
				cadastra.bt_fecha_dialogo.connect_clicked(move|_|{

					&cb_tipo_coord_clone2.append_text(&cadastra_clone.ent_dialog_modelo.get_text().unwrap().to_string());
					&ent_latitude_clone2.set_text(&cadastra_clone.ent_dialog_latitude.get_text().unwrap().to_string());
					&ent_longitude_clone2.set_text(&cadastra_clone.ent_dialog_longitude.get_text().unwrap().to_string());
					cadastra_clone.dialog.destroy();
				});

				//personal.pers_latitude = cadastra.ent_dialog_latitude.get_text().unwrap().to_string();

				cadastra.dialog.run();
				//cadastra
			});
		}
		
		window.connect_delete_event(move |_,_| {
			main_quit();
			Inhibit(false)
		});

		bt_fechar.connect_clicked(move |_| {
			main_quit();
			Inhibit(false);
		});

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
	        cb_tipo_coord,
	        //dialog,
			bt_teste,
			bt_ad
		}
	}
}

pub struct Cadastra {
	pub dialog:					Dialog,
	pub ent_dialog_modelo:		Entry,
	pub ent_dialog_latitude:	Entry,
	pub ent_dialog_longitude:	Entry,
	pub bt_fecha_dialogo:		Button,
	pub bt_preencher:			Button
}

impl Cadastra {
	fn new() -> Rc<Self> {
		let glade_src = include_str!("ui.glade");
		let glade = gtk::Builder::new_from_string(glade_src);
		let dialog: gtk::Dialog = glade.get_object("dialog").unwrap();

		let ent_dialog_modelo: Entry = glade.get_object("ent_dialog_modelo").unwrap();
		let ent_dialog_latitude: Entry = glade.get_object("ent_dialog_latitude").unwrap();
		let ent_dialog_longitude: Entry = glade.get_object("ent_dialog_longitude").unwrap();
		let bt_fecha_dialogo: Button = glade.get_object("bt_fecha_dialogo").unwrap();
		let bt_preencher: Button = glade.get_object("bt_preencher").unwrap();

		//dialog.add(&bt_fecha_dialogo);

		let cadastra = Rc::new(Self {
			dialog,
			ent_dialog_modelo,
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
	app.window.show_all();

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


struct Teste {
	x: String,
	y: String
}

fn set_entrys (	//combo: Gtk::ComboBoxText,
				entry_latitude: &Entry,
				entry_longitude: &Entry,
				id: &str) {
	let utm = Teste { x: String::from(r"\d.\d{3}.\d{3},\d{1,3}"), y: String::from(r" \d{3}.\d{3},\d{1,3}")};
	match id {
		"0" => {
			
			//entry_latitude.set_text(&String::from(r"\d.\d{3}.\d{3},\d{1,3}")); // Latitude
			entry_latitude.set_text(&utm.x); // Latitude
			//entry_longitude.set_text(&String::from(r" \d{3}.\d{3},\d{1,3}"));	// Longitude
			entry_longitude.set_text(&utm.y);	// Longitude
		},
		"1" => {
			entry_latitude.set_text(&String::from(r"[+-]?[3-4]\d\.\d{6}"));
			entry_longitude.set_text(&String::from(r"[+-]?[0-2]\d\.\d{6}"));
		},
		"2" => {
			entry_latitude.set_text(&String::from(r"[0-2]\dS\s[0-5]\d'\s[0-5]\d"));
			entry_longitude.set_text(&String::from(r"[3-7]\dW\s[0-5]\d'\s[0-5]\d"));
		},
		&_ => {
			println!("Não há nenhum padrão selecionado");
		} 
	
	}
}


fn carrega_perfis () -> std::io::Result<(), String> {
	let mut file = File::open("perfis.json")?;
	file
}
