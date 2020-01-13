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


/// TESTS

#[test]
fn cmu_katakana_test() {
    assert_eq!("エラン", cmu_katakana("aaron"));
}

#[test]
fn cmu_hiragana_test() {
    assert_eq!("がべん", cmu_hiragana("Gavin"));
}

#[test]
fn test_hiragana_vowel_inputs() {
    assert_eq!("あえいおう", to_hiragana("aeiou", false).unwrap());
}

#[test]
fn test_hiragana_capital_vowel_inputs() {
    assert_eq!("おう", to_hiragana("OU", false).unwrap());
}

#[test]
fn test_hiragana_open_syllables() {
    assert_eq!("きつね", to_hiragana("kitsune", false).unwrap());
}

#[test]
fn test_hiragana_capital_open_syllables() {
    assert_eq!("きつね", to_hiragana("KiTsuNE", false).unwrap());
}

#[test]
fn test_hiragana_closed_final_syllable() {
    assert_eq!("ごおん", to_hiragana("goon", false).unwrap());
}

#[test]
fn test_hiragana_closed_syllables() {
    assert_eq!("はんど", to_hiragana("hando", false).unwrap());
}

#[test]
fn test_hiragana_geminates() {
    assert_eq!("がっこう", to_hiragana("gakkou", false).unwrap());
}

#[test]
fn test_hiragana_multiple_words_with_whitespace() {
    assert_eq!(
        "おはよう ございます !",
        to_hiragana("ohayou gozaimasu !", false).unwrap()
    );
}

#[test]
fn test_hiragana_digraphs() {
    assert_eq!(
        "がっこう で いっしょうに",
        to_hiragana("gakkou de isshouni", false).unwrap()
    );
}

#[test]
fn test_katakana_vowel_inputs() {
    assert_eq!("アエイオウ", to_katakana("aeiou", false).unwrap());
}

#[test]
fn test_katakana_capital_vowel_inputs() {
    assert_eq!("オウ", to_katakana("OU", false).unwrap());
}

#[test]
fn test_katakana_open_syllables() {
    assert_eq!("キツネ", to_katakana("kitsune", false).unwrap());
}

#[test]
fn test_katakana_capital_open_syllables() {
    assert_eq!("キツネ", to_katakana("KItsUNe", false).unwrap());
}

#[test]
fn test_katakana_closed_final_syllable() {
    assert_eq!("ハンド", to_katakana("hando", false).unwrap());
}

#[test]
fn test_katakana_geminates() {
    assert_eq!("ガッコウ", to_katakana("gakkou", false).unwrap());
}

#[test]
fn test_katakana_multiple_words_with_whitespace() {
    assert_eq!(
        "オハヨウ ゴザイマス !",
        to_katakana("ohayou gozaimasu !", false).unwrap()
    );
}

#[test]
fn test_katakana_digraphs() {
    assert_eq!(
        "ガッコウ デ イッショウニ",
        to_katakana("gakkou de isshouni", false).unwrap()
    );
}

#[test]
fn test_katakana_long_vowel() {
    assert_eq!(
        "オーキーナ チャーハン",
        to_katakana("ookiina chaahan", false).unwrap()
    );
}

#[test]
fn test_roomaji_hiragana_open_syllable() {
    assert_eq!("kitsune", to_roomaji_hiragana("きつね").unwrap());
}

#[test]
fn test_roomaji_hiragana_closed_syllable() {
    assert_eq!("hando", to_roomaji_hiragana("はんど").unwrap());
}

#[test]
fn test_roomaji_hiragana_geminates() {
    assert_eq!("gakkou", to_roomaji_hiragana("がっこう").unwrap());
}

#[test]
fn test_roomaji_hiragana_multiple_words_with_whitespace() {
    assert_eq!("mai neemu isu maiku", to_roomaji_hiragana("まい ねえむ いす まいく").unwrap());
}

#[test]
fn test_roomaji_hiragana_digraphs() {
    assert_eq!("jon myuu", to_roomaji_hiragana("じょん みゅう").unwrap());
}

#[test]
fn test_roomaji_katakana_open_syllable() {
    assert_eq!("kitsune", to_roomaji_katakana("キツネ").unwrap());
}

#[test]
fn test_roomaji_katakana_closed_syllable() {
    assert_eq!("hando", to_roomaji_katakana("ハンド").unwrap());
}

#[test]
fn test_roomaji_katakana_geminates() {
    assert_eq!("gakkou", to_roomaji_katakana("ガッコウ").unwrap());
}

#[test]
fn test_roomaji_katakana_multiple_words_with_whitespace() {
    assert_eq!("mai neemu isu maiku", to_roomaji_katakana("マイ ネーム イス マイク").unwrap());
}

#[test]
fn test_roomaji_katakana_digraphs() {
    assert_eq!("jon myuu", to_roomaji_katakana("ジョン ミュー").unwrap());
}


#[test]
fn cmu_dict_tests() {
    let cmumap = make_cmu_map();
    let japmap = make_jap_map();
    
    assert_eq!(
        eng_to_jap("AARON", &cmumap, &japmap),
        vec![
            "E".to_string(),
            "R".to_string(), 
            "A".to_string(), 
            "N".to_string(),
        ],
        "AARON failed."
    );
}