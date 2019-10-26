#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod handlers;

use handlers::{
    index::*,
    hello_world::*,
    upload::*,
    catcher::*
};

fn main() {
    rocket::ignite()
        .register(
            catchers![
                not_found
            ],
        )
        .mount("/", 
            routes![
                hello_world,
                index,
                hello_world_no_last_name,
                upload
            ]
        ).launch();
}