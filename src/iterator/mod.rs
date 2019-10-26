use std::io::{Read};
use std::fs::{File};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Chunk {
  data: Vec<u8>,
  hash: String,
}

pub struct HashedChunker<R> where R:Read {
  pub source: R,
}

impl<R> Iterator for HashedChunker<R> where R: Read {
  // type Item = Chunk<[u8; 1024 * 1024]>;
  type Item = Chunk;
  fn next(&mut self) -> Option<Self::Item> {
    let mut buffer = [0; 2 * 1024*1024];
    let res = self.source.read(&mut buffer);
    match res {
      Ok(count) => {
        // format!("{:16x}", buffer[..10; 10]);
        if count > 0 {
            let slice = &buffer[..count];
            let vec = slice.to_vec();
            let mut hasher = DefaultHasher::new();
            hasher.write(&slice);
            let res = hasher.finish();
            let str = format!("{:16x}", res).trim().to_string();
            print!("{:?}\n", str);
            let c = Chunk{ data: vec, hash: str };
            Some(c)
        } else {
          None
        }
      },
      Err(_e) => None,
    }
  }
}

// let f = File::open("/home/yurik/Downloads/robo3t-1.3.1-linux-x86_64-7419c406.tar.gz").unwrap();
// let chunker = HashedChunker {
//   source: f
// };
// for _val in chunker {
//   println!("Got: chunk");
// }
