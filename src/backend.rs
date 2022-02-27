use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::string::String;

use csv::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub fn analisa_texto(
    uri_entrada: &PathBuf,

    uri_saida: &PathBuf,
    expressao_n: String,
    expressao_e: String,
) -> bool {
    let texto = fs::read_to_string(uri_entrada);
    let text = &String::from(texto.unwrap());

    println!("Conteúdo do texto {}", &text); // O texto memorial está sendo lido

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
        vetor3.push(x.replace(".", ""));
    }

    let mut vetor4 = Vec::with_capacity(VEC_SIZE);
    for x in vetor2.iter_mut() {
        vetor4.push(x.replace(".", ""));
    }

	if vetor3.len() == vetor4.len() {
    	gera_csv(vetor3, vetor4, uri_saida.to_path_buf())
    	    .expect("Não foi possível utilizar a uri informada pela função Dados::new()");
		true
	} else {
		false
	}
}

pub fn gera_csv(vec: Vec<String>, vec1: Vec<String>, uri2: PathBuf) -> Result<()> {
    let mut wtr = Writer::from_path(uri2)?;
    wtr.write_record(&["Latitude", "Longitude"])?;

    for i in 0..vec.len() {
        wtr.write_record(&[vec[i].as_str(), vec1[i].as_str()])
            .expect("Não foi possível gravar os dados do vetor");
    }

    wtr.flush()?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expressoes {
    pub latitude: String,
    pub longitude: String,
}

pub fn salva_perfis(serializado: String) -> std::io::Result<()> {
    let mut file = File::create("perfis.yaml")?;
    file.write_all(&serializado.as_bytes())?;
    println!("Perfis salvos no disco.");

    Ok(())
}

pub fn carrega_perfis() -> std::io::Result<String> {
    Ok(fs::read_to_string("perfis.yaml")?)
}

#[allow(dead_code)]
fn serializa(map: &BTreeMap<String, Expressoes>) -> String {
    serde_json::to_string(&map).unwrap()
}

pub fn serializa_yaml(map: &BTreeMap<String, Expressoes>) -> String {
    serde_yaml::to_string(&map).unwrap()
}

#[allow(dead_code)]
pub fn desserializa(serializado: String) -> BTreeMap<String, Expressoes> {
    serde_json::from_str(&serializado).unwrap()
}

pub fn desserializa_yaml(serializado: String) -> BTreeMap<String, Expressoes> {
    serde_yaml::from_str(&serializado).unwrap()
}

pub fn popula_perfis() -> BTreeMap<String, Expressoes> {
    let utm = Expressoes {
        latitude: String::from(r"\d.\d{3}.\d{3},\d{1,3}"),
        longitude: String::from(r" \d{3}.\d{3},\d{1,3}"),
    };

    let decimal = Expressoes {
        latitude: String::from(r"[+-]?[3-4]\d\.\d{6}"),
        longitude: String::from(r"[+-]?[0-2]\d\.\d{6}"),
    };

    let gms = Expressoes {
        latitude: String::from(r"[0-2]\dS\s[0-5]\d'\s[0-5]\d"),

        longitude: String::from(r"[3-7]\dW\s[0-5]\d'\s[0-5]\d"),
    };
    let gms_neg = Expressoes {
        latitude: String::from(r#"[+-]?[0-2]\d°[0-9]\d'[0-9]\d,\d{3}""#),
        longitude: String::from(r#"[+-]?[3-4]\d°[0-9]\d'[0-9]\d[,.]?\d{3}""#),
    };
    let mut perfis = BTreeMap::new();

    perfis.insert("UTM".to_string(), utm);
    perfis.insert("Decimal".to_string(), decimal);
    perfis.insert("Graus, minutos e segundos".to_string(), gms);
    perfis.insert("GMS Completo".to_string(), gms_neg);
    perfis
}

pub fn remove_perfil(perfil_atual: String, perfis: &Rc<RefCell<BTreeMap<String, Expressoes>>>) {
    let mut map: RefMut<_> = perfis.borrow_mut();
    map.remove(perfil_atual.as_str());
}
