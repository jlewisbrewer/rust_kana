// Copyright 2018 Jason Brewer and Gavin Megson

// Kana Transliterator

// Takes a string input and converts it to a Japanese hiragana or katakana
// output



use std::collections::HashMap;

struct Kana {
    hiragana : HashMap<String, String>
}

impl Kana {
    // A new kana struct will be initilized with a hiragana hashmap
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
                        "\u{306F}", "\u{3072}", "\u{3075}", "\u{3078}", "\u{307A}",
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
                        "\u{308F}", "\u{3090}", "NOT USED", "\u{3090}", "\u{3091}"];
    
        let mut u_iter = unicodekeys.iter();
        let mut s_iter = syllabary.iter();
        for s in &syllabary {
            hiragana_table.insert(s_iter.next().unwrap().to_string(), u_iter.next().unwrap().to_string());
        }

    
        Kana{hiragana: hiragana_table}
    }
}


fn main() {
    let kanatable = Kana::new();

    println!("{:?}", kanatable.hiragana);

}
