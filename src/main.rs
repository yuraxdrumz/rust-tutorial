#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod handlers;
mod iterator;

use std::fs::{File};
use iterator::{ HashedChunker };
use handlers::{
    index::*,
    hello_world::*,
    upload::*,
    catcher::*
};

fn main() {
    let f = File::open("/home/yurik/Downloads/robo3t-1.3.1-linux-x86_64-7419c406.tar.gz").unwrap();
    let chunker = HashedChunker {
        source: f
    };
    for _val in chunker {
        // _val is u8 array
        println!("Got: chunk");
    }
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