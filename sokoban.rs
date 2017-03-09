#![crate_type = "bin"]

#![allow(unused_must_use)]

//extern crate native;
extern crate libc;

use std::from_str::{FromStr};
use std::io::{File};
use std::io::stdio::{stdin};
use std::path::{Path};
use std::os;

use sokoboard::{SokoBoard};
use sokoannotatedboard::{SokoAnnotatedBoard, do_sylvan};

mod raw;
mod bdd;
mod sokoboard;
mod sokoannotatedboard;

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

  let annotated = SokoAnnotatedBoard::fromSokoBoard(board);
  do_sylvan(&annotated);
}

