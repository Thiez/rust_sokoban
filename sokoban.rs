#![no_uv]
#![allow(unused_must_use)]

extern crate native;
extern crate libc;

use std::from_str::{FromStr};
use std::io::{File, Writer};
use std::io::stdio::{stdin, stdout, stderr};
use std::path::{Path};
use std::os;

use sokoboard::{SokoBoard, Field};
use sokoannotatedboard::{SokoAnnotatedBoard, AnnotatedField};

mod raw;
mod bdd;
mod sokoboard;
mod sokoannotatedboard;


#[start]
fn start(argc: int, argv: **u8) -> int {
  native::start(argc,argv,proc(){
      main()
  })
}

fn init() {
  unsafe {
    raw::raw_init();
  }
  println!("Init sylvan");
}

fn main() {
  let args = os::args();
  let contents;
  if args.len() > 1 {
    contents = File::open(&Path::new(args[1].as_slice())).read_to_str();
    println!("Reading from file.");
  } else {
    contents = stdin().read_to_str();
    println!("Reading from stdin.");
  }

  let board: SokoBoard = FromStr::from_str( contents.unwrap() )
            .expect("Invalid sokoban board");
  println!("The screen:\n{}\nEnd of screen.", board);

  init();

  let annotated = SokoAnnotatedBoard::fromSokoBoard(board);

}

