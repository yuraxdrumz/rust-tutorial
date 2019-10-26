
use rocket::data::{ Data, DataStream };
use std::io::{Result};
use reqwest;
use std::fs::{File};
use iterator::{ HashedChunker };

#[post("/upload", data = "<file>")]
pub fn upload(file: Data) -> Result<String> {
  let data_stream = file.open();

  let chunker = HashedChunker {
      source: data_stream
  };

  for val in chunker {
      // _val is u8 array
      println!("Got: {:?}", val.hash);
  }
  // let client = reqwest::Client::new();
  // let res = client.post("http://httpbin.org/anything")
  //     .body(file)
  //     .send()?;
  // let f = file.
  // f.take(10);
  // let chunker = HashedChunker {
  //     source: f
  // };
  // for _val in chunker {
  //     // _val is u8 array
  //     println!("Got: chunk");
  // }
  // let res = client.post("https://httpbin.org/anything").body().send()?;
  // file.stream_to(&mut stdout()).map(|n| format!("Wrote {} bytes.", n))
  // file.stream_to_file("/tmp/upload.txt").map(|n| n.to_string())
  return Ok(String::new());
}