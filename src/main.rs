extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use hyper::Url;
use hyper::Client;

use rustc_serialize::json;

static SPELL_URL: &'static str = "http://speller.yandex.net/services/spellservice.json/checkText";

#[derive(RustcDecodable)]
struct Error {
    code: i32,
    pos:  i32,
    row:  i32,
    col:  i32,
    len:  i32,
    word: String,
    s:    Vec<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let options: String = self.s.join(", ");
        write!(f, "{} [{}:{} code:{}]. Hints: {}", self.word, self.row + 1, self.col, self.code, options)
    }
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

    let mut url = Url::parse(SPELL_URL).unwrap();
    url.query_pairs_mut()
        .append_pair("text", text.as_ref());

    let mut response = client.get(url).send().unwrap();
    assert_eq!(response.status, hyper::Ok);

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    body
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = read_text(args[1].to_string());
    let errors: Vec<Error> = json::decode(&check_text(text)).unwrap();

    for error in &errors {
        println!("{}", error);
    }
}
