extern crate hyper;

use std::env;
use std::io::Read;

use hyper::Url;
use hyper::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let client = Client::new();

    let mut url = Url::parse("http://speller.yandex.net/services/spellservice.json/checkText").unwrap();
    url.query_pairs_mut()
        .append_pair("text", args[1].as_ref());

    let mut res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
