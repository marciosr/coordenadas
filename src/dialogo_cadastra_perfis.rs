extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::{Window, Entry, Button};

use std::rc::Rc;

pub struct Cadastra {
	pub dialog:					Window,
	pub ent_dialog_perfil:		Entry,
	pub ent_dialog_latitude:	Entry,
	pub ent_dialog_longitude:	Entry,
	pub bt_fecha_dialogo:		Button,
	pub bt_preencher:			Button
}

impl Cadastra {
	pub fn new() -> Rc<Self> {
		let glade_src = include_str!("dialogo_cadastra_perfis.glade");
		let glade = gtk::Builder::new_from_string(glade_src);
		let dialog: gtk::Window = glade.get_object("dialog").expect("Não foi possivel encontrar o widget");

		let ent_dialog_perfil: Entry = glade.get_object("ent_dialog_perfil").expect("Não foi possivel encontrar o widget");
		let ent_dialog_latitude: Entry = glade.get_object("ent_dialog_latitude").expect("Não foi possivel encontrar o widget");
		let ent_dialog_longitude: Entry = glade.get_object("ent_dialog_longitude").expect("Não foi possivel encontrar o widget");
		let bt_fecha_dialogo: Button = glade.get_object("bt_fecha_dialogo").expect("Não foi possivel encontrar o widget");
		let bt_preencher: Button = glade.get_object("bt_preencher").expect("Não foi possivel encontrar o widget");

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
