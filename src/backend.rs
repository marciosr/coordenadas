extern crate gtk;
//extern crate regex;


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


pub fn analisa_texto (uri_entrada: PathBuf, uri_saida: PathBuf, expressao_n: String,  expressao_e: String) -> Result<()> {
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

pub fn gera_csv (vec: Vec<String>,vec1: Vec<String>, uri2: PathBuf) -> Result<()> {
	let mut wtr = Writer::from_path(uri2)?;
	wtr.write_record(&["Latitude","Longitude"])?;

	for i in 0..vec.len() {
		wtr.write_record(&[vec[i].as_str(),vec1[i].as_str()]).expect("Não foi possível gravar os dados do vetor");
	}

	wtr.flush()?;
	Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expressoes {
	latitude: String,
	longitude: String
}

pub fn set_entrys (	entry_latitude: &Entry,
				entry_longitude: &Entry,
				nome_perfil: &String,
				perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {
	let map = perfis.borrow();

	let expressoes = map.get(nome_perfil).unwrap();
	entry_latitude.set_text(&expressoes.latitude);
	entry_longitude.set_text(&expressoes.longitude);
}

pub fn adiciona_perfil (	perfil_n: String,
						latitude_n: &String,
						longitude_n: &String,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	let expressoes = Expressoes { latitude: latitude_n.to_string(), longitude: longitude_n.to_string()};

	let mut map: RefMut<_> = perfis.borrow_mut();
	println!("O conteúdo dos perfils dentro da função adiciona perfi é: {:?}", map); // Para testes!
	map.insert(perfil_n, expressoes);
}

pub fn salva_perfis (serializado: String) -> std::io::Result<()> {
	let mut file = File::create("perfis.yaml")?;
	file.write_all(&serializado.as_bytes())?;

	Ok(())
}

pub fn carrega_perfis () -> std::io::Result<(String)> {
	let file = fs::read_to_string("perfis.yaml")?;
	Ok(file)
}

#[allow(dead_code)]
fn serializa (map: &BTreeMap<String, Expressoes>) -> String {
	let serializado = serde_json::to_string(&map).unwrap();
	serializado
}

pub fn serializa_yaml (map: &BTreeMap<String, Expressoes>) -> String {
	let serializado = serde_yaml::to_string(&map).unwrap();
	serializado
}

#[allow(dead_code)]
pub fn desserializa (serializado: String) -> BTreeMap<String, Expressoes> {
	let desserializado: BTreeMap<String, Expressoes> = serde_json::from_str(&serializado).unwrap();
	desserializado
}

pub fn desserializa_yaml (serializado: String) -> BTreeMap<String, Expressoes> {
	let desserializado: BTreeMap<String, Expressoes> = serde_yaml::from_str(&serializado).unwrap();
	desserializado
}

pub fn inicia_combo (	combo: &ComboBoxText,
					perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>
				) {
	let map = perfis.borrow();
	for (key, _value) in map.iter() {
			println!("Inciando... {:?}", key);
		combo.append_text(&key);
	}
}

pub fn atualiza_campos (	nome_perfil: String,
						ent_latitude: &Entry,
						ent_longitude: &Entry,
						perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>> ) {

	set_entrys(&ent_latitude, &ent_longitude, &String::from(nome_perfil), perfis);
}

pub fn popula_perfis () -> BTreeMap<String, Expressoes> {
	let utm = Expressoes { latitude: String::from(r"\d.\d{3}.\d{3},\d{1,3}"), longitude: String::from(r" \d{3}.\d{3},\d{1,3}")};
	let decimal = Expressoes { latitude: String::from(r"[+-]?[3-4]\d\.\d{6}"), longitude: String::from(r"[+-]?[0-2]\d\.\d{6}")};
	let gms = Expressoes { latitude: String::from(r"[0-2]\dS\s[0-5]\d'\s[0-5]\d"), longitude: String::from(r"[3-7]\dW\s[0-5]\d'\s[0-5]\d")};

	let mut perfis = BTreeMap::new();

	perfis.insert("UTM".to_string(), utm);
	perfis.insert("Decimal".to_string(), decimal);
	perfis.insert("Graus, minutos e segundos".to_string(), gms);
	perfis
}

pub fn remove_perfil (	combo: &ComboBoxText,
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