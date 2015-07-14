extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Payload {
    name: String,
    pi: f32,
    best_number: u32,
    right_now: String,
}

fn main() {
    let client = Client::new();

    let mut res = client.get("http://localhost:9292/")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let payload: Payload = json::decode(&body).unwrap();

    println!("{:?}", payload);
}
