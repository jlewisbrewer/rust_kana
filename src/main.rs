// Copyright 2018 Jason Brewer and Gavin Megson

// Kana Transliterator

// Takes a string input in the Latin 1 character set and converts 
//it to a Japanese hiragana or katakana output
#[macro_use]
extern crate lazy_static; // 1.0.2

use std::env;
use std::collections::HashMap;

lazy_static!{

    static ref HIRAGANA : HashMap<String, String> = {
        initialize_hiragana()
    };
    static ref KATAKANA: HashMap<String, String> = {
        initialize_katakana()
    };
}


// Function to create a vector that contains all Japanese
// syllables.
fn initialize_japanese_syllables() -> Vec<String> {

        // Japanese vowels
        let vowels = ["a", "i", "u", "e", "o"];
        // Japanese consonants
        let consonants = ["k", "g", "s", "z", "t", "d", "n", "h",
                        "b", "p", "m", "y", "r", "w"];
                        
        // Japanese digraphs
        let digraphs_vowels = ["a", "u", "o"];
        let digraph_consonants = ["ky", "sh", "ch", "nq", "hy", 
                        "my", "ry", "gy", "j", "by", "py"];
    
    
        let mut syllabary = Vec::new();
        for v in &vowels{
            syllabary.push(format!("{}", v));
        }    
        for c in &consonants {
            for v in &vowels {
                syllabary.push(format!("{}{}", c,v));
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
            let index = syllabary.iter().position(|r| r == &syllable.to_string()).unwrap();
            syllabary[index] = replace_with[i].to_string();
        }
    syllabary
    
}

// Function that map Japanese syllables to corresponding hiragana
// values.
fn initialize_hiragana() -> HashMap<String, String> {
        let mut hiragana_table = HashMap::new();

        let syllabary = initialize_japanese_syllables();


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
                        // voiceless velar stop digraph
                        "\u{304D}\u{3083}",   "\u{304D}\u{3085}",  "\u{304D}\u{3087}",
                        // voiceless alveolar sibilant digraph
                         "\u{3057}\u{3083}",  "\u{3057}\u{3085}",  "\u{3057}\u{3087}",
                        // voiceless alveolar affricate digraph
                         "\u{3061}\u{3083}",  "\u{3061}\u{3085}",  "\u{3061}\u{3087}",
                        // alveolar nasal digraph
                         "\u{306B}\u{3083}",  "\u{306B}\u{3085}",  "\u{306B}\u{3087}", 
                        // voiceless glottal aproximant digraph
                         "\u{3072}\u{3083}",  "\u{3072}\u{3085}",  "\u{3072}\u{3087}",
                        // bilabial nasal digraph
                         "\u{307F}\u{3083}",  "\u{307F}\u{3085}",  "\u{307F}\u{3087}",
                        // lateral approximant digraph
                         "\u{308A}\u{3083}",  "\u{308A}\u{3085}",  "\u{308A}\u{3087}",
                        // voiced velar stop digraph
                         "\u{304E}\u{3083}",  "\u{304E}\u{3085}",  "\u{304E}\u{3087}",
                        // voiced alveolar affricate digraph
                         "\u{3058}\u{3083}",  "\u{3058}\u{3085}",  "\u{3058}\u{3087}",
                        // voiced bilabial stop digraph
                         "\u{3073}\u{3083}",  "\u{3073}\u{3085}",  "\u{3073}\u{3087}",
                        // voiceless bilabial stop digraph
                         "\u{3074}\u{3083}",  "\u{3074}\u{3085}",  "\u{3074}\u{3087}",
                        // final nasal
                        "\u{3093}",
                        // small tsu for geminates
                        "\u{3063}"];

                        
                        
        let mut u_iter = unicodekeys.iter();
        let mut s_iter = syllabary.iter();
        for _s in &syllabary {
            hiragana_table.insert(s_iter.next().unwrap().to_string(), u_iter.next().unwrap().to_string());
        }

        // These syllables are phonologically possible but do not have associated kana.
        hiragana_table.remove("yi");
        hiragana_table.remove("ye");
        hiragana_table.remove("wu");
        
        hiragana_table
}

fn initialize_katakana() -> HashMap<String, String> {
        let mut katakana_table = HashMap::new();
        

        let mut syllabary = initialize_japanese_syllables();
        
        syllabary.push("l".to_string());
        // A listing of the unicode values for hiragana
 
        // A listing of the unicode values for hiragana
        let unicodekeys = [ // vowels
                        "\u{30A2}", "\u{30A4}", "\u{30A6}", "\u{30A8}", "\u{30AA}",
                        // voiceless velar stops
                        "\u{30AB}", "\u{30AD}", "\u{30AF}", "\u{30B1}", "\u{30B3}",
                        // voiced velar stops
                        "\u{30AC}", "\u{30AE}", "\u{30B0}", "\u{30B2}", "\u{30B4}",
                        // voiceless alveolar sibilants
                        "\u{30B5}", "\u{30B7}", "\u{30B9}", "\u{30BB}", "\u{30BD}",
                        // voiced alveolar sibilants
                        "\u{30B6}", "\u{30B8}", "\u{30BA}", "\u{30BC}", "\u{30BE}",
                        // voiceless alveolar stops
                        "\u{30BF}", "\u{30C1}", "\u{30C4}", "\u{30C6}", "\u{30C8}",
                        // voiced alveolar stops
                        "\u{30C0}", "\u{30C2}", "\u{30C5}", "\u{30C7}", "\u{30C9}",
                        // alveolar nasals
                        "\u{30CA}", "\u{30CB}", "\u{30CC}", "\u{30CD}", "\u{30CE}",
                        // voiceless glottal fricative
                        "\u{30CF}", "\u{30D2}", "\u{30D5}", "\u{30D8}", "\u{30DB}",
                        // voiced bilabial stop
                        "\u{30D0}", "\u{30D3}", "\u{30D6}", "\u{30D9}", "\u{30DC}",
                        // voiceless bilabial stop
                        "\u{30D1}", "\u{30D4}", "\u{30D7}", "\u{30DA}", "\u{30DD}",
                        // bilabila nasal
                        "\u{30DE}", "\u{30DF}", "\u{30E0}", "\u{30E1}", "\u{30E2}",
                        // palatal approximant
                        "\u{30E4}", "NOT USED", "\u{30E6}", "NOT USED", "\u{30E8}",
                        // lateral flap
                        "\u{30E9}", "\u{30EA}", "\u{30EB}", "\u{30EC}", "\u{30ED}",
                        // bilabial approximant
                        "\u{30EF}", "\u{30F0}", "NOT USED", "\u{30F0}", "\u{30F1}",
                        // voiceless velar stop digraph
                        "\u{30AD}\u{30E3}",   "\u{30AD}\u{30E5}",  "\u{30AD}\u{30E7}",
                        // voiceless alveolar sibilant digraph
                         "\u{30B7}\u{30E3}",  "\u{30B7}\u{30E5}",  "\u{30B7}\u{30E7}",
                        // voiceless alveolar affricate digraph
                         "\u{30C1}\u{30E3}",  "\u{30C1}\u{30E5}",  "\u{30C1}\u{30E7}",
                        // alveolar nasal digraph
                         "\u{30CB}\u{30E3}",  "\u{30CB}\u{30E5}",  "\u{30CB}\u{30E7}", 
                        // voiceless glottal aproximant digraph
                         "\u{30D2}\u{30E3}",  "\u{30D2}\u{30E5}",  "\u{30D2}\u{30E7}",
                        // bilabial nasal digraph
                         "\u{30DF}\u{30E3}",  "\u{30DF}\u{30E5}",  "\u{30DF}\u{30E7}",
                        // lateral approximant digraph
                         "\u{30EA}\u{30E3}",  "\u{30EA}\u{30E5}",  "\u{30EA}\u{30E7}",
                        // voiced velar stop digraph
                         "\u{30AE}\u{30E3}",  "\u{30AE}\u{30E5}",  "\u{30AE}\u{30E7}",
                        // voiced alveolar affricate digraph
                         "\u{30B8}\u{30E3}",  "\u{30B8}\u{30E5}",  "\u{30B8}\u{30E7}",
                        // voiced bilabial stop digraph
                         "\u{30D3}\u{30E3}",  "\u{30D3}\u{30E5}",  "\u{30D3}\u{30E7}",
                        // voiceless bilabial stop digraph
                         "\u{30D4}\u{30E3}",  "\u{30D4}\u{30E5}",  "\u{30D4}\u{30E7}",
                        // final nasal
                        "\u{30F3}",
                        // small tsu for geminates
                        "\u{30C3}",
                        // choonpu for long vowels
                        "\u{30FC}"];
                        
                        
        let mut u_iter = unicodekeys.iter();
        let mut s_iter = syllabary.iter();
        for _s in &syllabary {
            katakana_table.insert(s_iter.next().unwrap().to_string(), u_iter.next().unwrap().to_string());
        }

        // These syllables are phonologically possible but do not have associated kana.
        katakana_table.remove("yi");
        katakana_table.remove("ye");
        katakana_table.remove("wu");
        
        katakana_table
}

fn to_japanese_syllables(input: &str) -> Vec<String> {
    let input = input.to_lowercase();

    // This vector is used to store the syllables for the input string
    let mut syllables = Vec::new();
    // Vowel vector for comparison
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    // Possible geminate characters
    let geminates = vec!['k', 't', 'p', 'g', 'd', 'b', 's', 'z', 'c'];
    // Possible digraphs
    let digraph = vec!["ky", "sh", "ch", "nq", "hy", 
                        "my", "ry", "gy", "by", "py"];
    
    let mut tempsyllable = "".to_string();
    let mut tempdigraph = "".to_string();
    let mut prevnasal = false;
    let mut prevgeminate = false;

    for c in input.chars(){
        tempsyllable.push(c);
        tempdigraph.push(c);

        // This check adds non-alphabetic chars to the syllable vector.
        if !c.is_alphabetic(){
            prevnasal = false;
            syllables.push(c.to_string());
            tempsyllable = "".to_string();
        }

        // This checks to see if it's a digraph
        if digraph.contains(&tempdigraph.as_str()){
            if prevgeminate{
                syllables.pop();
            }
            if prevnasal{
                syllables.pop();
            }
            tempsyllable = "".to_string();
            tempsyllable.push_str(&tempdigraph);
            prevgeminate = false;
            prevnasal = false;

        }
        // Japanese syllables can end in final -n, so it needs to be checked.
        if c == 'n'{
            prevnasal = true;
            syllables.push(c.to_string());
        } else if !vowels.contains(&c) && prevnasal {
            prevnasal = false;
            tempsyllable = "".to_string();
            tempsyllable.push(c);
        }
        // This checks the geminate array and sets geminate flag to test for gemination.
        if !vowels.contains(&c) && prevgeminate{

            let tempchar = syllables.pop().unwrap();

  
            if tempchar == c.to_string() {
                tempsyllable = "".to_string();
                syllables.push("g".to_string());
                tempsyllable.push(c);
                prevgeminate = false;
                continue;
            }
        }
        if geminates.contains(&c){
            prevgeminate = true;
            syllables.push(c.to_string());
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
        
        if tempdigraph.len() == 2 {
            tempdigraph = "".to_string();
            tempdigraph.push(c);
        }
    }    
    //println!("{:?}", &syllables);
    syllables
}

fn to_hiragana(input : &str) -> String {
    let mut output = "".to_string();
    let syllables = to_japanese_syllables(input);
    // After the syllables have been parsed, we can get the kana values for them
     for c in &syllables {
        let temp = c.to_string();
        let mut tempchar = c.chars();
        if !tempchar.next().unwrap().is_alphabetic(){
            output.push_str(&temp);
        } else {
            output.push_str(HIRAGANA.get(&temp).unwrap());
        }
    }
    output
}

fn to_katakana(input : &str) -> String {
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

        if !c.chars().next().unwrap().is_alphabetic(){
            output.push_str(&temp);
        } else {
            output.push_str(KATAKANA.get(&temp).unwrap());
        }
        last_vowel = c.chars().last().unwrap();

    }

    output
}


// fn to_roomaji(input : &str) -> String {
//     let mut output = "".to_string();
//     for c in input.chars() {
//         for (key, value) in hiragana.iter(){
//             if value.contains(c){
//                 output.push_str(&key);
//             }
//         }
//     }
//     output
// }



fn main() {

    
    let args: Vec<String> = env::args().collect();


    println!("In hiragana: {:?}", to_hiragana(&args[1]));
    println!("In katakana: {:?}", to_katakana(&args[1]));
    // println!("Roomaji from hiragana: {}", to_roomaji(&to_hiragana(&args[1])));

}


#[test]
fn test_hiragana_vowel_inputs() {
    assert_eq!("あえいおう", to_hiragana("aeiou")); 
}

#[test]
fn test_hiragana_capital_vowel_inputs() {
    assert_eq!("おう", to_hiragana("OU"));
}

#[test]
fn test_hiragana_open_syllables() {
    assert_eq!("きつね", to_hiragana("kitsune"));
}

#[test]
fn test_hiragana_capital_open_syllables() {
    assert_eq!("きつね", to_hiragana("KiTsuNE"));
}

#[test]
fn test_hiragana_closed_final_syllable() {
    assert_eq!("ごおん", to_hiragana("goon"));
}

#[test]
fn test_hiragana_closed_syllables() {
    assert_eq!("はんど", to_hiragana("hando"));
}

#[test]
fn test_hiragana_geminates() {
    assert_eq!("がっこう", to_hiragana("gakkou"));
}

#[test]
fn test_multiple_words_with_whitespace(){
    assert_eq!("おはよう ございます !", to_hiragana("ohayou gozaimasu !"));
}

#[test]
fn test_hiragana_digraphs(){
    assert_eq!("がっこう で いっしょうに", to_hiragana("gakkou de isshouni"));
}
