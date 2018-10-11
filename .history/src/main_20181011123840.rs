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
use mongodb::{Client, ThreadedClient, CommandResult};
use mongodb::db::ThreadedDatabase;
// use bson::{Bson, Document};
// use bson::oid::ObjectId;

const MONGO_URL: &'static str = "localhost";
const MONGO_PORT: u16 = 27017;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub _id: bson::oid::ObjectId,
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub picture: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchUserById {
    pub _id: bson::oid::ObjectId
}

fn log_query_duration(client: Client, command_result: &CommandResult) {
    match command_result {
        &CommandResult::Success { duration, ref command_name, .. } => {
            println!("Command {:#} took {} nanoseconds.", command_name, duration);
        },
        _ => println!("Failed to execute command."),
    }
}

fn main() {
    let mut client = Client::connect(MONGO_URL, MONGO_PORT)
    .expect("failed to initialize client");

    client.add_completion_hook(log_query_duration).unwrap();


    let users_coll = client.db("Dashboard").collection("users");
    let cursor = users_coll.find(None, None).ok().expect("Failed to execute find.");

    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    let serialized = serde_json::to_string(&docs).unwrap();

    let deserialized: Vec<User> = serde_json::from_str(&serialized).unwrap();
    println!("{:#?}", deserialized);

    // Match the user id to an bson ObjectId
    let id = bson::oid::ObjectId::with_string("sdasdasdasdasda").unwrap();

    println!("id: {}", id);

    // let user = users_coll.find(doc! { "_id" => oid}, None).unwrap();


    rocket::ignite()
    .mount("/", routes![
        handlers::index::index,
        handlers::hello_world::hello_world
    ])
    .launch();
}