// Copyright 2018 Jason Brewer and Gavin Megson
//
//!  Kana Transliterator
//!
//!  Takes a string input in the Latin 1 character set and converts
//! it to a Japanese hiragana or katakana output.
//!
//!  # Examples
//!
//!  ````
//!  let input = "kana";
//!  assert_eq!(to_hiragana(input).unwrap(), "かな");
//!  assert_eq!(to_katakana(input).unwrap(), "カナ");
//!  ````
//!
#[macro_use]
extern crate lazy_static; // 1.0.2

use std::env;
use std::process;
use std::collections::HashMap;

mod kana;
use kana::trans::{to_hiragana, to_katakana, to_roomaji_hiragana, to_roomaji_katakana};
use kana::cmu::{cmu_hiragana, cmu_katakana};
use kana::conv::{initialize_hiragana, initialize_hiragana_keys, initialize_katakana,
    initialize_katakana_keys, initilize_roomaji};

/// Global constant hashmaps that map Latin 1 syllables to Japanese
/// kana in unicode.
lazy_static! {
    pub static ref HIRAGANA: HashMap<String, String> = { initialize_hiragana() };
    pub static ref KATAKANA: HashMap<String, String> = { initialize_katakana() };
    pub static ref ROOMAJI_HIRAGANA: HashMap<String, String> = { initilize_roomaji(initialize_hiragana_keys) };
    pub static ref ROOMAJI_KATAKANA: HashMap<String, String> = { initilize_roomaji(initialize_katakana_keys) };
    pub static ref HIRAGANA_KEYS: Vec<&'static str> = { initialize_hiragana_keys() };
    pub static ref KATAKANA_KEYS: Vec<&'static str> = { initialize_katakana_keys() };
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Incorrect number of command line arguemnts, please type README in command line for details");
        process::exit(1);
    }

    let option = &args[1];
    match option.as_str() {
        "hiragana" => println!("{}", to_hiragana(&args[2], false).expect("Unable to parse input")),
        "katakana" => println!("{}", to_katakana(&args[2], false).expect("Unable to parse input")),
        "roomaji_hiragana" => println!("{}", to_roomaji_hiragana(&args[2]).expect("Unable to parse input")),
        "roomaji_katakana" => println!("{}", to_roomaji_katakana(&args[2]).expect("Unable to parse input")),
        "cmu_hiragana"   => println!("{}",cmu_hiragana(&args[2])),
        "cmu_katakana"   => println!("{}",cmu_katakana(&args[2])),
        _ => println!("Incorrect command line argument, please see README for details."),
    }
}