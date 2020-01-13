use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use kana::trans::*;


/// Make map from CMU phones into japanese-like phones.
/// The CMU phones list has been edited to include
/// the japanese-like phones.
/// 
/// Takes a file of whitespace-delineated key/value
/// pairs separated in lines and creates a HashMap.
fn make_jap_map() -> HashMap<String, String> {
    let mut phones = HashMap::new();
    let file = File::open("cmu_data/cmuphones.txt".to_string())
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
    let file = File::open("cmu_data/cmudict-0.7b.txt".to_string())
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
pub fn cmu_hiragana(word: &str) -> String {
    let cmu = make_cmu_map();
    let jap = make_jap_map();
    let temp: String = eng_to_jap(word,&cmu,&jap).join("");
    to_hiragana(temp.as_str(), true).expect("to_hiragana from cmu_hiragana failed")
}

pub fn cmu_katakana(word: &str) -> String {
    let cmu = make_cmu_map();
    let jap = make_jap_map();
    let temp: String = eng_to_jap(word,&cmu,&jap).join("");
    to_katakana(temp.as_str(), true).expect("to_katakana from cmu_katakana failed")
}

/// Tests

#[test]
fn cmu_katakana_test() {
    assert_eq!("エラン", cmu_katakana("aaron"));
}

#[test]
fn cmu_hiragana_test() {
    assert_eq!("がべん", cmu_hiragana("Gavin"));
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