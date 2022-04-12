extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, Entry, Button, HeaderBar};
use std::rc::Rc;

pub struct Cadastra {
	pub dialog:					Window,
	pub header:					HeaderBar,
	pub ent_dialog_perfil:		Entry,
	pub ent_dialog_latitude:	Entry,
	pub ent_dialog_longitude:	Entry,
	pub bt_fecha_dialogo:		Button,
	pub bt_preencher:			Button
}

impl Cadastra {
	pub fn new() -> Rc<Self> {
		let ui_src = include_str!("dialogo_cadastra_perfis.ui");
		let ui = gtk::Builder::from_string(ui_src);

		get_widget!(ui, Window,		dialog);
		get_widget!(ui, HeaderBar,	header);
		get_widget!(ui, Entry, 		ent_dialog_perfil);
		get_widget!(ui, Entry, 		ent_dialog_latitude);
		get_widget!(ui, Entry, 		ent_dialog_longitude);
		get_widget!(ui, Button, 	bt_fecha_dialogo);
		get_widget!(ui, Button, 	bt_preencher);

		{
			let dialog_clone = dialog.clone();
			bt_fecha_dialogo.connect_clicked (move |_| {
				dialog_clone.close();
			});
		}

		let cadastra = Rc::new(Self {
			dialog,
			header,
			ent_dialog_perfil,
			ent_dialog_latitude,
			ent_dialog_longitude,
			bt_fecha_dialogo,
			bt_preencher
		});
		cadastra
	}
}
