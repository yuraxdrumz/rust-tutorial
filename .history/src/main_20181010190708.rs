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
use mongodb::{ Client };
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;
use bson::{Bson, Document};
use bson::oid::ObjectId;


fn main() {
    let client = Client::new("localhost").ok().expect("Error in connection");

    rocket::ignite()
    .mount("/", routes![
        handlers::index::index,
        handlers::hello_world::hello_world
    ])
    .launch();
}