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
mod to_kana {
    extern crate lazy_static; // 1.0.2

    use std::collections::HashMap;

    /// Global constant hashmaps that map Latin 1 syllables to Japanese
    /// kana in unicode.
    lazy_static! {
        static ref HIRAGANA: HashMap<String, String> = { initialize_hiragana() };
        static ref KATAKANA: HashMap<String, String> = { initialize_katakana() };
    }

    /// Returns a vector of strings that represent Japanese syllables
    fn initialize_japanese_syllables() -> Vec<String> {
        // Japanese vowels
        let vowels = ["a", "i", "u", "e", "o"];
        // Japanese consonants
        let consonants = [
            "k", "g", "s", "z", "t", "d", "n", "h", "b", "p", "m", "y", "r", "w",
        ];

        // Japanese digraphs
        let digraphs_vowels = ["a", "u", "o"];
        let digraph_consonants = [
            "ky", "sh", "ch", "nq", "hy", "my", "ry", "gy", "j", "by", "py",
        ];

        let mut syllabary = Vec::new();
        for v in &vowels {
            syllabary.push(format!("{}", v));
        }
        for c in &consonants {
            for v in &vowels {
                syllabary.push(format!("{}{}", c, v));
            }
        }
        for c in &digraph_consonants {
            for v in &digraphs_vowels {
                syllabary.push(format!("{}{}", c, v));
            }
        }
        // Final n
        syllabary.push("n".to_string());
        // This is the flag for gemination
        syllabary.push("g".to_string());

        // There are a number of syllables that are transcribed differently by
        // convention. This loop is designed to find their phonemic representations
        // and replace them. The iter is sourced from stack overflow,
        // https://stackoverflow.com/questions/30558246/how-do-i-find-the-index-of-an-element-in-an-array-vector-or-slice
        // posted by Mathieu David.
        let to_replace = ["si", "ti", "tu", "hu", "zi"];
        let replace_with = ["shi", "chi", "tsu", "fu", "ji"];
        for i in 0..5 {
            let syllable = to_replace[i];
            let index = syllabary
                .iter()
                .position(|r| r == &syllable.to_string())
                .unwrap();
            syllabary[index] = replace_with[i].to_string();
        }
        syllabary
    }

    /// Returns a hashmap of Latin 1 strings as keys and Japanese
    /// hiragana strings as values.
    fn initialize_hiragana() -> HashMap<String, String> {
        let mut hiragana_table = HashMap::new();

        let syllabary = initialize_japanese_syllables();

        let unicodekeys = [
            // vowels
            "\u{3042}",
            "\u{3044}",
            "\u{3046}",
            "\u{3048}",
            "\u{304A}",
            // voiceless velar stops
            "\u{304B}",
            "\u{304D}",
            "\u{304F}",
            "\u{3051}",
            "\u{3053}",
            // voiced velar stops
            "\u{304C}",
            "\u{304E}",
            "\u{3050}",
            "\u{3052}",
            "\u{3054}",
            // voiceless alveolar sibilants
            "\u{3055}",
            "\u{3057}",
            "\u{3059}",
            "\u{305B}",
            "\u{305D}",
            // voiced alveolar sibilants
            "\u{3056}",
            "\u{3058}",
            "\u{305A}",
            "\u{305C}",
            "\u{305E}",
            // voiceless alveolar stops
            "\u{305F}",
            "\u{3061}",
            "\u{3064}",
            "\u{3066}",
            "\u{3068}",
            // voiced alveolar stops
            "\u{3060}",
            "\u{3062}",
            "\u{3065}",
            "\u{3067}",
            "\u{3069}",
            // alveolar nasals
            "\u{306A}",
            "\u{306B}",
            "\u{306C}",
            "\u{306D}",
            "\u{306E}",
            // voiceless glottal fricative
            "\u{306F}",
            "\u{3072}",
            "\u{3075}",
            "\u{3078}",
            "\u{307B}",
            // voiced bilabial stop
            "\u{3070}",
            "\u{3073}",
            "\u{3076}",
            "\u{3079}",
            "\u{307C}",
            // voiceless bilabial stop
            "\u{3071}",
            "\u{3074}",
            "\u{3077}",
            "\u{307A}",
            "\u{307D}",
            // bilabila nasal
            "\u{307E}",
            "\u{307F}",
            "\u{3080}",
            "\u{3081}",
            "\u{3082}",
            // palatal approximant
            "\u{3084}",
            "NOT USED",
            "\u{3086}",
            "NOT USED",
            "\u{3088}",
            // lateral flap
            "\u{3089}",
            "\u{308A}",
            "\u{308B}",
            "\u{308C}",
            "\u{308D}",
            // bilabial approximant
            "\u{308F}",
            "\u{3090}",
            "NOT USED",
            "\u{3090}",
            "\u{3091}",
            // voiceless velar stop digraph
            "\u{304D}\u{3083}",
            "\u{304D}\u{3085}",
            "\u{304D}\u{3087}",
            // voiceless alveolar sibilant digraph
            "\u{3057}\u{3083}",
            "\u{3057}\u{3085}",
            "\u{3057}\u{3087}",
            // voiceless alveolar affricate digraph
            "\u{3061}\u{3083}",
            "\u{3061}\u{3085}",
            "\u{3061}\u{3087}",
            // alveolar nasal digraph
            "\u{306B}\u{3083}",
            "\u{306B}\u{3085}",
            "\u{306B}\u{3087}",
            // voiceless glottal aproximant digraph
            "\u{3072}\u{3083}",
            "\u{3072}\u{3085}",
            "\u{3072}\u{3087}",
            // bilabial nasal digraph
            "\u{307F}\u{3083}",
            "\u{307F}\u{3085}",
            "\u{307F}\u{3087}",
            // lateral approximant digraph
            "\u{308A}\u{3083}",
            "\u{308A}\u{3085}",
            "\u{308A}\u{3087}",
            // voiced velar stop digraph
            "\u{304E}\u{3083}",
            "\u{304E}\u{3085}",
            "\u{304E}\u{3087}",
            // voiced alveolar affricate digraph
            "\u{3058}\u{3083}",
            "\u{3058}\u{3085}",
            "\u{3058}\u{3087}",
            // voiced bilabial stop digraph
            "\u{3073}\u{3083}",
            "\u{3073}\u{3085}",
            "\u{3073}\u{3087}",
            // voiceless bilabial stop digraph
            "\u{3074}\u{3083}",
            "\u{3074}\u{3085}",
            "\u{3074}\u{3087}",
            // final nasal
            "\u{3093}",
            // small tsu for geminates
            "\u{3063}",
        ];

        let mut u_iter = unicodekeys.iter();
        let mut s_iter = syllabary.iter();
        for _s in &syllabary {
            hiragana_table.insert(
                s_iter.next().unwrap().to_string(),
                u_iter.next().unwrap().to_string(),
            );
        }

        // These syllables are phonologically possible but do not have associated kana.
        hiragana_table.remove("yi");
        hiragana_table.remove("ye");
        hiragana_table.remove("wu");

        hiragana_table
    }

    /// Returns a hashmap of Latin 1 strings as keys and Japanese
    /// katakana strings as values.
    fn initialize_katakana() -> HashMap<String, String> {
        let mut katakana_table = HashMap::new();

        let mut syllabary = initialize_japanese_syllables();

        // This value is used to represent long vowels in order to map to the katakana
        // choonpu kana.
        syllabary.push("l".to_string());

        // A listing of the unicode values for katakana
        let unicodekeys = [
            // vowels
            "\u{30A2}",
            "\u{30A4}",
            "\u{30A6}",
            "\u{30A8}",
            "\u{30AA}",
            // voiceless velar stops
            "\u{30AB}",
            "\u{30AD}",
            "\u{30AF}",
            "\u{30B1}",
            "\u{30B3}",
            // voiced velar stops
            "\u{30AC}",
            "\u{30AE}",
            "\u{30B0}",
            "\u{30B2}",
            "\u{30B4}",
            // voiceless alveolar sibilants
            "\u{30B5}",
            "\u{30B7}",
            "\u{30B9}",
            "\u{30BB}",
            "\u{30BD}",
            // voiced alveolar sibilants
            "\u{30B6}",
            "\u{30B8}",
            "\u{30BA}",
            "\u{30BC}",
            "\u{30BE}",
            // voiceless alveolar stops
            "\u{30BF}",
            "\u{30C1}",
            "\u{30C4}",
            "\u{30C6}",
            "\u{30C8}",
            // voiced alveolar stops
            "\u{30C0}",
            "\u{30C2}",
            "\u{30C5}",
            "\u{30C7}",
            "\u{30C9}",
            // alveolar nasals
            "\u{30CA}",
            "\u{30CB}",
            "\u{30CC}",
            "\u{30CD}",
            "\u{30CE}",
            // voiceless glottal fricative
            "\u{30CF}",
            "\u{30D2}",
            "\u{30D5}",
            "\u{30D8}",
            "\u{30DB}",
            // voiced bilabial stop
            "\u{30D0}",
            "\u{30D3}",
            "\u{30D6}",
            "\u{30D9}",
            "\u{30DC}",
            // voiceless bilabial stop
            "\u{30D1}",
            "\u{30D4}",
            "\u{30D7}",
            "\u{30DA}",
            "\u{30DD}",
            // bilabila nasal
            "\u{30DE}",
            "\u{30DF}",
            "\u{30E0}",
            "\u{30E1}",
            "\u{30E2}",
            // palatal approximant
            "\u{30E4}",
            "NOT USED",
            "\u{30E6}",
            "NOT USED",
            "\u{30E8}",
            // lateral flap
            "\u{30E9}",
            "\u{30EA}",
            "\u{30EB}",
            "\u{30EC}",
            "\u{30ED}",
            // bilabial approximant
            "\u{30EF}",
            "\u{30F0}",
            "NOT USED",
            "\u{30F0}",
            "\u{30F1}",
            // voiceless velar stop digraph
            "\u{30AD}\u{30E3}",
            "\u{30AD}\u{30E5}",
            "\u{30AD}\u{30E7}",
            // voiceless alveolar sibilant digraph
            "\u{30B7}\u{30E3}",
            "\u{30B7}\u{30E5}",
            "\u{30B7}\u{30E7}",
            // voiceless alveolar affricate digraph
            "\u{30C1}\u{30E3}",
            "\u{30C1}\u{30E5}",
            "\u{30C1}\u{30E7}",
            // alveolar nasal digraph
            "\u{30CB}\u{30E3}",
            "\u{30CB}\u{30E5}",
            "\u{30CB}\u{30E7}",
            // voiceless glottal aproximant digraph
            "\u{30D2}\u{30E3}",
            "\u{30D2}\u{30E5}",
            "\u{30D2}\u{30E7}",
            // bilabial nasal digraph
            "\u{30DF}\u{30E3}",
            "\u{30DF}\u{30E5}",
            "\u{30DF}\u{30E7}",
            // lateral approximant digraph
            "\u{30EA}\u{30E3}",
            "\u{30EA}\u{30E5}",
            "\u{30EA}\u{30E7}",
            // voiced velar stop digraph
            "\u{30AE}\u{30E3}",
            "\u{30AE}\u{30E5}",
            "\u{30AE}\u{30E7}",
            // voiced alveolar affricate digraph
            "\u{30B8}\u{30E3}",
            "\u{30B8}\u{30E5}",
            "\u{30B8}\u{30E7}",
            // voiced bilabial stop digraph
            "\u{30D3}\u{30E3}",
            "\u{30D3}\u{30E5}",
            "\u{30D3}\u{30E7}",
            // voiceless bilabial stop digraph
            "\u{30D4}\u{30E3}",
            "\u{30D4}\u{30E5}",
            "\u{30D4}\u{30E7}",
            // final nasal
            "\u{30F3}",
            // small tsu for geminates
            "\u{30C3}",
            // choonpu for long vowels
            "\u{30FC}",
        ];

        let mut u_iter = unicodekeys.iter();
        let mut s_iter = syllabary.iter();
        for _s in &syllabary {
            katakana_table.insert(
                s_iter.next().unwrap().to_string(),
                u_iter.next().unwrap().to_string(),
            );
        }

        // These syllables are phonologically possible but do not have associated kana.
        katakana_table.remove("yi");
        katakana_table.remove("ye");
        katakana_table.remove("wu");

        katakana_table
    }

    /// Returns a vector of strings where each string represents a Japanese syllable
    ///
    /// #Arguments
    ///
    /// * `input` - A str slice that needs to be parsed into Japanese syllables
    ///
    /// # Example
    ///
    /// ```
    /// let test_input = "toto";
    /// assert_eq!(to_japanese_syllables(test_input), "["to", "to"]);
    /// ```
    ///
    fn to_japanese_syllables(input: &str) -> Vec<String> {
        let input = input.to_lowercase();

        // This vector is used to store the syllables for the input string
        let mut syllables = Vec::new();
        // Vowel array for comparison
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        // Possible geminate characters
        let geminates = ['k', 't', 'p', 'g', 'd', 'b', 's', 'z', 'c'];
        // Possible digraphs
        let digraph = ["ky", "sh", "ch", "nq", "hy", "my", "ry", "gy", "by", "py"];

        let mut tempsyllable = "".to_string();
        let mut tempdigraph = "".to_string();
        let mut prevnasal = false;
        let mut prevgeminate = false;

        for c in input.chars() {
            tempsyllable.push(c);
            tempdigraph.push(c);

            // This check adds non-alphabetic chars to the syllable vector.
            if !c.is_alphabetic() {
                prevnasal = false;
                syllables.push(c.to_string());
                tempsyllable = "".to_string();
            }

            // This checks to see if it's a digraph
            if digraph.contains(&tempdigraph.as_str()) {
                if prevgeminate {
                    syllables.pop();
                }
                if prevnasal {
                    syllables.pop();
                }
                tempsyllable = "".to_string();
                tempsyllable.push_str(&tempdigraph);
                prevgeminate = false;
                prevnasal = false;
            }
            // Japanese syllables can end in final -n, so it needs to be checked.
            if c == 'n' {
                prevnasal = true;
                syllables.push(c.to_string());
            } else if !vowels.contains(&c) && prevnasal {
                prevnasal = false;
                tempsyllable = "".to_string();
                tempsyllable.push(c);
            }
            // This checks the geminate array and sets geminate flag to test for gemination.
            if !vowels.contains(&c) && prevgeminate {
                let tempchar = syllables.pop().unwrap();

                if tempchar == c.to_string() {
                    tempsyllable = "".to_string();
                    syllables.push("g".to_string());
                    tempsyllable.push(c);
                    prevgeminate = false;
                    continue;
                }
            }
            if geminates.contains(&c) {
                prevgeminate = true;
                syllables.push(c.to_string());
            }

            if vowels.contains(&c) {
                if prevgeminate {
                    syllables.pop();
                }

                if prevnasal {
                    syllables.pop();
                    prevnasal = false;
                }
                prevgeminate = false;
                syllables.push(tempsyllable);
                tempsyllable = "".to_string();
            }

            if tempdigraph.len() >= 2 {
                tempdigraph = "".to_string();
                tempdigraph.push(c);
            }
        }
        syllables
    }

    ///  Returns a result that gives a string in hiragana on success.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that will be converted to hiragana
    ///
    ///  # Examples
    ///
    ///  ```
    ///  let input = "kana";
    ///  assert_eq!(to_hiragana(input).unwrap(), "かな");
    ///  ````
    ///
    pub fn to_hiragana(input: &str) -> Result<String, String> {
        let mut output = "".to_string();
        let syllables = to_japanese_syllables(input);
        // After the syllables have been parsed, we can get the kana values for them
        for c in &syllables {
            let temp = c.to_string();
            let mut tempchar = c.chars();
            if !tempchar.next().unwrap().is_alphabetic() {
                output.push_str(&temp);
            } else {
                let result = HIRAGANA.get(&temp);
                match result {
                    Some(_) => output.push_str(result.unwrap()),
                    None => return Err("Unable to parse input".to_string()),
                }
            }
        }
        Ok(output)
    }

    ///  Returns a result that gives a string in katakana on success.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that will be converted to katakana
    ///
    ///  # Examples
    ///
    ///  ```
    ///  let input = "kana";
    ///  assert_eq!(to_katakana(input).unwrap(), "カナ");
    ///  ````
    ///
    pub fn to_katakana(input: &str) -> Result<String, String> {
        let mut output = "".to_string();

        let syllables = to_japanese_syllables(input);

        let mut last_vowel = ' ';
        // After the syllables have been parsed, we can get the kana values for them
        for c in &syllables {
            let mut temp = c.to_string();
            if &last_vowel.to_string() == c {
                // This retrieves the choonpu used for long vowels in katakana.
                temp = "l".to_string();
            }

            if !c.chars().next().unwrap().is_alphabetic() {
                output.push_str(&temp);
            } else {
                let result = KATAKANA.get(&temp);
                match result {
                    Some(_) => output.push_str(result.unwrap()),
                    None => return Err("Unable to parse input".to_string()),
                }
            }
            last_vowel = c.chars().last().unwrap();
        }
        Ok(output)
    }
}

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

/// Make map from CMU phones into japanese-like phones.
/// The CMU phones list has been edited to include
/// the japanese-like phones.
/// 
/// Takes a file of whitespace-delineated key/value
/// pairs separated in lines and creates a HashMap.
fn make_jap_map() -> HashMap<String, String> {
    let mut phones = HashMap::new();
    let file = File::open("cmuphones.txt".to_string())
                    .expect("file not found: cmuphones.txt");
    for line in BufReader::new(file).lines() {
        let next = line.expect("out of lines MJM");
        let substrings: Vec<&str> = next
                            .split_whitespace()
                            .take(2)
                            .collect();
        phones.insert(substrings[0].to_string()
                     ,substrings[1].to_string());

    }
    phones
}

/// Make hashmap from CMU phones to Japanese phones.
/// The Carnegie Mellon University phonetic dictionary
/// was downloaded, and non-alphabetic keys were deleted.
fn make_cmu_map() -> HashMap<String, String> {
    let mut phones = HashMap::new();
    let file = File::open("cmudict-0.7b.txt".to_string())
                    .expect("file not found cmudict-0.7b.txt");
    let mut reader = BufReader::new(file).lines();
    while let Some(line) = reader.next() {
        let next = line.expect("out of lines MCM");
        let substrings: Vec<&str> = next.splitn(2,' ')
                                        .collect();

        let head = substrings[0].to_string();
        let tail = substrings[1].to_string();

       
        phones.insert(head,tail);

    }
    phones
}


/// Take english word, get cmu phones, return japanese-ready phones.
/// These phones should be easily parsed by katakana/hiragana function.
fn eng_to_jap(word: &str,
        cmu_map: & HashMap<String, String>,
        jap_map: & HashMap<String, String>)
        -> Vec<String> {

    let mut out = Vec::new();
    let mut eng_phones: Vec<&str> = 
                        cmu_map.get(&word.to_uppercase())
                               .expect("not found in cmu")
                               .split_whitespace()
                               .collect()
                               ;
    for mut phone in &mut eng_phones {
        // Remove stress markers. (Numbers)
        let mut has_stress = false;
        for character in phone.chars() {
            if character.is_alphabetic() == false {
                has_stress = true;
            }
        }

        let mut phonestring = phone.to_string();
        if has_stress == true {
            // Stress marker is always at end.
            phonestring.pop();
        }
        
        out.push(jap_map.get(phonestring.as_str())
                        .expect("not in japanese-ready dictionary")
                        .to_string());
    }
    out
}

/// Full process functions, from english->CMU->japanese
fn cmu_hiragana(word: &str) -> String {
    let cmu = make_cmu_map();
    let jap = make_jap_map();
    let temp: String = eng_to_jap(word,&cmu,&jap).join("");
    to_hiragana(temp.as_str()).expect("to_hiragana from cmu_hiragana failed")
}

fn cmu_katakana(word: &str) -> String {
    let cmu = make_cmu_map();
    let jap = make_jap_map();
    let temp: String = eng_to_jap(word,&cmu,&jap).join("");
    to_katakana(temp.as_str()).expect("to_katakana from cmu_katakana failed")
}


#[macro_use]
extern crate lazy_static; // 1.0.2

use std::env;
use std::process;
use to_kana::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Incorrect number of command line arguemnts, please type README in command line for details");
        process::exit(1);
    }
    let option = &args[1];
    match option.as_str() {
        "hiragana" => println!("{}", to_hiragana(&args[2]).unwrap()),
        "katakana" => println!("{}", to_katakana(&args[2]).unwrap()),
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
    assert_eq!("あえいおう", to_hiragana("aeiou").unwrap());
}

#[test]
fn test_hiragana_capital_vowel_inputs() {
    assert_eq!("おう", to_hiragana("OU").unwrap());
}

#[test]
fn test_hiragana_open_syllables() {
    assert_eq!("きつね", to_hiragana("kitsune").unwrap());
}

#[test]
fn test_hiragana_capital_open_syllables() {
    assert_eq!("きつね", to_hiragana("KiTsuNE").unwrap());
}

#[test]
fn test_hiragana_closed_final_syllable() {
    assert_eq!("ごおん", to_hiragana("goon").unwrap());
}

#[test]
fn test_hiragana_closed_syllables() {
    assert_eq!("はんど", to_hiragana("hando").unwrap());
}

#[test]
fn test_hiragana_geminates() {
    assert_eq!("がっこう", to_hiragana("gakkou").unwrap());
}

#[test]
fn test_hiragana_multiple_words_with_whitespace() {
    assert_eq!(
        "おはよう ございます !",
        to_hiragana("ohayou gozaimasu !").unwrap()
    );
}

#[test]
fn test_hiragana_digraphs() {
    assert_eq!(
        "がっこう で いっしょうに",
        to_hiragana("gakkou de isshouni").unwrap()
    );
}

#[test]
fn test_katakana_vowel_inputs() {
    assert_eq!("アエイオウ", to_katakana("aeiou").unwrap());
}

#[test]
fn test_katakana_capital_vowel_inputs() {
    assert_eq!("オウ", to_katakana("OU").unwrap());
}

#[test]
fn test_katakana_open_syllables() {
    assert_eq!("キツネ", to_katakana("kitsune").unwrap());
}

#[test]
fn test_katakana_capital_open_syllables() {
    assert_eq!("キツネ", to_katakana("KItsUNe").unwrap());
}

#[test]
fn test_katakana_closed_final_syllable() {
    assert_eq!("ハンド", to_katakana("hando").unwrap());
}

#[test]
fn test_katakana_geminates() {
    assert_eq!("ガッコウ", to_katakana("gakkou").unwrap());
}

#[test]
fn test_katakana_multiple_words_with_whitespace() {
    assert_eq!(
        "オハヨウ ゴザイマス !",
        to_katakana("ohayou gozaimasu !").unwrap()
    );
}

#[test]
fn test_katakana_digraphs() {
    assert_eq!(
        "ガッコウ デ イッショウニ",
        to_katakana("gakkou de isshouni").unwrap()
    );
}

#[test]
fn test_katakana_long_vowel() {
    assert_eq!(
        "オーキーナ チャーハン",
        to_katakana("ookiina chaahan").unwrap()
    );
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
