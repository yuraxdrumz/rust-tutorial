#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
#[macro_use]
extern crate serde_derive; // 1.0.70
extern crate serde; // 1.0.70
extern crate serde_json; // 1.0.24
extern crate rocket;
extern crate rocket_contrib;

mod handlers;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson::{Bson, Document};
use bson::oid::ObjectId;

const MONGO_URL: &'static str = "localhost";
const MONGO_PORT: u16 = 27017;



fn main() {
    let client = Client::connect(MONGO_URL, MONGO_PORT)
    .expect("failed to initialize client");

    let users_coll = client.db("Dashboard").collection("users");
    let cursor = users_coll.find(doc!{}, None).ok().expect("Failed to execute find.");

    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    let serialized = serde_json::to_string(&docs).unwrap();

    println!("{}", serialized);
    rocket::ignite()
    .mount("/", routes![
        handlers::index::index,
        handlers::hello_world::hello_world
    ])
    .launch();
}