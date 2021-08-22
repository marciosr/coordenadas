extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, Entry, Button};

use std::rc::Rc;

pub struct Cadastra {
	pub dialog:								Window,
	pub ent_dialog_perfil:		Entry,
	pub ent_dialog_latitude:	Entry,
	pub ent_dialog_longitude:	Entry,
	pub bt_fecha_dialogo:			Button,
	pub bt_preencher:					Button
}

impl Cadastra {
	pub fn new() -> Rc<Self> {
		let glade_src = include_str!("dialogo_cadastra_perfis.ui");
		let glade = gtk::Builder::from_string(glade_src);
		let dialog: gtk::Window = glade.object("dialog").expect("Não foi possivel encontrar o widget");

		let ent_dialog_perfil: Entry = glade.object("ent_dialog_perfil").expect("Não foi possivel encontrar o widget");
		let ent_dialog_latitude: Entry = glade.object("ent_dialog_latitude").expect("Não foi possivel encontrar o widget");
		let ent_dialog_longitude: Entry = glade.object("ent_dialog_longitude").expect("Não foi possivel encontrar o widget");
		let bt_fecha_dialogo: Button = glade.object("bt_fecha_dialogo").expect("Não foi possivel encontrar o widget");
		let bt_preencher: Button = glade.object("bt_preencher").expect("Não foi possivel encontrar o widget");

		{
			let dialog_clone = dialog.clone();
			bt_fecha_dialogo.connect_clicked (move |_| {
				dialog_clone.close();
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
