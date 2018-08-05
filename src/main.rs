// Copyright 2018 Jason Brewer and Gavin Megson

// Kana Transliterator

// Takes a string input in the Latin 1 character set and converts 
//it to a Japanese hiragana or katakana output



use std::collections::HashMap;

struct Kana {
    hiragana : HashMap<String, String>
}

impl Kana {
    fn new() -> Kana {
        let mut hiragana_table = HashMap::new();
    
        // Japanese vowels
        let vowels = ["a", "i", "u", "e", "o"];
        // Japanese consonants
        let consonants = ["k", "g", "s", "z", "t", "d", "n", "h",
                        "b", "p", "m", "y", "r", "w"];
    
    
        let mut syllabary = Vec::new();
        for v in &vowels{
            syllabary.push(format!("{}", v));
        }    
        for c in &consonants {
            for v in &vowels {
                syllabary.push(format!("{}{}", c,v));
            }
        }
        // Final n
        syllabary.push("n".to_string());
        // This is the flag for gemination
        syllabary.push("g".to_string());
    

        // A listing of the unicode values for hiragana
        let unicodekeys = [ // vowels
                        "\u{3042}", "\u{3044}", "\u{3046}", "\u{3048}", "\u{304A}",
                        // voiceless velar stops
                        "\u{304B}", "\u{304D}", "\u{304F}", "\u{3051}", "\u{3053}",
                        // voiced velar stops
                        "\u{304C}", "\u{304E}", "\u{3050}", "\u{3052}", "\u{3054}",
                        // voiceless alveolar sibilants
                        "\u{3055}", "\u{3057}", "\u{3059}", "\u{305B}", "\u{305D}",
                        // voiced alveolar sibilants
                        "\u{3056}", "\u{3058}", "\u{305A}", "\u{305C}", "\u{305E}",
                        // voiceless alveolar stops
                        "\u{305F}", "\u{3061}", "\u{3064}", "\u{3066}", "\u{3068}",
                        // voiced alveolar stops
                        "\u{3060}", "\u{3062}", "\u{3065}", "\u{3067}", "\u{3069}",
                        // alveolar nasals
                        "\u{306A}", "\u{306B}", "\u{306C}", "\u{306D}", "\u{306E}",
                        // voiceless glottal fricative
                        "\u{306F}", "\u{3072}", "\u{3075}", "\u{3078}", "\u{307B}",
                        // voiced bilabial stop
                        "\u{3070}", "\u{3073}", "\u{3076}", "\u{3079}", "\u{307C}",
                        // voiceless bilabial stop
                        "\u{3071}", "\u{3074}", "\u{3077}", "\u{307A}", "\u{307D}",
                        // bilabila nasal
                        "\u{307E}", "\u{307F}", "\u{3080}", "\u{3081}", "\u{3082}",
                        // palatal approximant
                        "\u{3084}", "NOT USED", "\u{3086}", "NOT USED", "\u{3088}",
                        // lateral flap
                        "\u{3089}", "\u{308A}", "\u{308B}", "\u{308C}", "\u{308D}",
                        // bilabial approximant
                        "\u{308F}", "\u{3090}", "NOT USED", "\u{3090}", "\u{3091}",
                        // final nasal
                        "\u{3093}",
                        // small tsu for geminates
                        "\u{3063}"];
                        
                        
        let mut u_iter = unicodekeys.iter();
        let mut s_iter = syllabary.iter();
        for s in &syllabary {
            hiragana_table.insert(s_iter.next().unwrap().to_string(), u_iter.next().unwrap().to_string());
        }

    
        Kana{hiragana: hiragana_table}
    }
}

fn to_hiragana(input : &str, table: &Kana) -> String {
    let mut output = "".to_string();
    
    let input = input.to_lowercase();
    let mut syllables = Vec::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let geminates = vec!['k', 't', 'p', 'g', 'd', 'b', 's', 'z'];
    let mut tempsyllable = "".to_string();
    let mut prevnasal = false;
    let mut prevgeminate = false;
    for c in input.chars(){
        tempsyllable.push(c);
        if c == 'n'{
            prevnasal = true;
            syllables.push(c.to_string());
        } else if !vowels.contains(&c) && prevnasal {
            prevnasal = false;
            tempsyllable = "".to_string();
            tempsyllable.push(c);
        }
        if geminates.contains(&c){
            prevgeminate = true;
            syllables.push(c.to_string());
        }
        if !vowels.contains(&c) && prevgeminate{
            prevgeminate = true;
            let tempchar = syllables.pop().unwrap();
  
            if tempchar == c.to_string() {
                tempsyllable = "".to_string();
                syllables.push("g".to_string());
                tempsyllable.push(c);
            }
        }
        if vowels.contains(&c){
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

    }    
    //println!("{:?}", &syllables);
    for c in &syllables {
        let temp = c.to_string();
        for (english, kana) in &table.hiragana {
            if &temp == english {
                output.push_str(&kana);
            }
        }
    }
    output
}


fn main() {
    let kanatable = Kana::new();

    println!("{:?}", kanatable.hiragana);

}


#[test]
fn test_hiragana_vowel_inputs() {
    let test_table = Kana::new();
    assert_eq!("あえいおう", to_hiragana("aeiou", &test_table)); 
}

#[test]
fn test_hiragana_capital_vowel_inputs() {
    let test_table = Kana::new();
    assert_eq!("おう", to_hiragana("OU", &test_table));
}

#[test]
fn test_hiragana_open_syllables() {
    let test_table = Kana::new();
    assert_eq!("きつね", to_hiragana("kitune", &test_table));
}

#[test]
fn test_hiragana_capital_open_syllables() {
    let test_table = Kana::new();
    assert_eq!("きつね", to_hiragana("KiTuNE", &test_table));
}

#[test]
fn test_hiragana_closed_final_syllable() {
    let test_table = Kana::new();
    assert_eq!("ごおん", to_hiragana("goon", &test_table));
}

#[test]
fn test_hiragana_closed_syllables() {
    let test_table = Kana::new();
    assert_eq!("はんど", to_hiragana("hando", &test_table));
}

#[test]
fn test_hiragana_geminates() {
    let test_table = Kana::new();
    assert_eq!("がっこう", to_hiragana("gakkou", &test_table));
}
