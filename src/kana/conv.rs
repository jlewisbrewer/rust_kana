use std::collections::HashMap;

use HIRAGANA_KEYS;
use KATAKANA_KEYS;


/// Returns a vector of strings that represent Japanese syllables
pub fn initialize_japanese_syllables() -> Vec<String> {
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
    syllabary.push("G".to_string());
    // This is a digraph used in mostly foreign words
    syllabary.push("je".to_string());

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

/// Returns an array of unicode strings to represent hiragana
pub fn initialize_hiragana_keys() -> Vec<&'static str> {
    let unicodekeys = vec![
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
        // foreign digraph
        "\u{3058}\u{3047}",
    ];

    unicodekeys
}

/// Returns a vector of katakana unicode string slices used in building the hashmap
pub fn initialize_katakana_keys() -> Vec<&'static str> {
    // A listing of the unicode values for katakana
    let unicodekeys = vec![
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
        // bilabial nasal
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
        // foreign digraph
        "\u{30B8}\u{30A7}",
        // choonpu for long vowels
        "\u{30FC}",
    ];

    unicodekeys 
}

/// Returns a hashmap of Latin 1 strings as keys and Japanese
/// hiragana strings as values.
pub fn initialize_hiragana() -> HashMap<String, String> {
    let mut hiragana_table = HashMap::new();

    let syllabary = initialize_japanese_syllables();

    let mut u_iter = HIRAGANA_KEYS.iter();
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

    // Additional mappings to correspond with English consonants
    hiragana_table.insert("b".to_string(), "\u{3076}".to_string());
    hiragana_table.insert("ch".to_string(), "\u{3061}".to_string());
    hiragana_table.insert("d".to_string(), "\u{3069}".to_string());
    hiragana_table.insert("z".to_string(), "\u{3058}".to_string());
    hiragana_table.insert("f".to_string(), "\u{3075}".to_string());
    hiragana_table.insert("g".to_string(), "\u{3050}".to_string());
    hiragana_table.insert("h".to_string(), "\u{3075}".to_string());
    hiragana_table.insert("j".to_string(), "\u{3058}".to_string());
    hiragana_table.insert("k".to_string(), "\u{304F}".to_string());
    hiragana_table.insert("r".to_string(), "\u{308B}".to_string());
    hiragana_table.insert("p".to_string(), "\u{307D}".to_string());
    hiragana_table.insert("s".to_string(), "\u{3059}".to_string());
    hiragana_table.insert("sh".to_string(), "\u{3057}".to_string());
    hiragana_table.insert("t".to_string(), "\u{3068}".to_string());
    hiragana_table.insert("si".to_string(), "\u{3057}".to_string());
    hiragana_table.insert("ti".to_string(), "\u{3061}".to_string());
    hiragana_table.insert("tu".to_string(), "\u{3064}".to_string());
    hiragana_table.insert("hu".to_string(), "\u{3075}".to_string());
    hiragana_table.insert("zi".to_string(), "\u{3058}".to_string());
    hiragana_table.insert("m".to_string(), "\u{3080}".to_string());

    

    hiragana_table
}

/// Returns a hashmap of Latin 1 strings as keys and Japanese
/// katakana strings as values.
pub fn initialize_katakana() -> HashMap<String, String> {
    let mut katakana_table = HashMap::new();

    let mut syllabary = initialize_japanese_syllables();

    // This value is used to represent long vowels in order to map to the katakana
    // choonpu kana.
    syllabary.push("L".to_string());

    let mut u_iter = KATAKANA_KEYS.iter();
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

     // Additional mappings that correspond to English consonants
    katakana_table.insert("b".to_string(), "\u{30D6}".to_string());
    katakana_table.insert("ch".to_string(), "\u{30C1}".to_string());
    katakana_table.insert("d".to_string(), "\u{30C9}".to_string());
    katakana_table.insert("z".to_string(), "\u{30B8}".to_string());
    katakana_table.insert("f".to_string(), "\u{30D5}".to_string());
    katakana_table.insert("g".to_string(), "\u{30B0}".to_string());
    katakana_table.insert("h".to_string(), "\u{30D5}".to_string());
    katakana_table.insert("j".to_string(), "\u{30B8}".to_string());
    katakana_table.insert("k".to_string(), "\u{30AF}".to_string());
    katakana_table.insert("r".to_string(), "\u{30EB}".to_string());
    katakana_table.insert("p".to_string(), "\u{30DD}".to_string());
    katakana_table.insert("s".to_string(), "\u{30B9}".to_string());
    katakana_table.insert("sh".to_string(), "\u{30B7}".to_string());
    katakana_table.insert("t".to_string(), "\u{30C8}".to_string());
    katakana_table.insert("si".to_string(), "\u{30B7}".to_string());
    katakana_table.insert("ti".to_string(), "\u{30C1}".to_string());
    katakana_table.insert("tu".to_string(), "\u{30C4}".to_string());
    katakana_table.insert("hu".to_string(), "\u{30D5}".to_string());
    katakana_table.insert("zi".to_string(), "\u{30B8}".to_string());
    katakana_table.insert("m".to_string(), "\u{30E0}".to_string());


    katakana_table
}

/// Returns a hashmap of kana characters to english syllables used for romanization
pub fn initilize_roomaji(initialize_kana : fn()-> Vec<&'static str>) -> HashMap<String, String> {
    let mut roomaji_table = HashMap::new();
    let kana_unicode = initialize_kana();
    let syllabary = initialize_japanese_syllables();

    let mut u_iter = kana_unicode.iter();
    let mut s_iter = syllabary.iter();
    for _s in &syllabary {
        roomaji_table.insert(
            u_iter.next().unwrap().to_string(),
            s_iter.next().unwrap().to_string(),
        );
    }

    roomaji_table

}