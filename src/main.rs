extern crate hyper;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use hyper::Url;
use hyper::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = read_text(args[1].to_string());
    println!("{}", check_text(text));
}

fn read_text(path: String) -> String {
    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't open file"),
        Ok(file) => file,
    };

    let mut text = String::new();
    file.read_to_string(&mut text);
    text
}

fn check_text(text: String) -> String {
    let client = Client::new();
    let mut url = Url::parse("http://speller.yandex.net/services/spellservice.json/checkText").unwrap();
    url.query_pairs_mut()
        .append_pair("text", text.as_ref());

    let mut response = client.get(url).send().unwrap();
    assert_eq!(response.status, hyper::Ok);

    let mut json = String::new();
    response.read_to_string(&mut json).unwrap();
    json
}
