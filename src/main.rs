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
        static ref ROOMAJI_HIRAGANA: HashMap<String, String> = { initilize_roomaji(initialize_hiragana_keys) };
        static ref ROOMAJI_KATAKANA: HashMap<String, String> = { initilize_roomaji(initialize_katakana_keys) };
        static ref HIRAGANA_KEYS: Vec<&'static str> = { initialize_hiragana_keys() };
        static ref KATAKANA_KEYS: Vec<&'static str> = { initialize_katakana_keys() };

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
    fn initialize_hiragana_keys() -> Vec<&'static str> {
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
    fn initialize_katakana_keys() -> Vec<&'static str> {
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
    fn initialize_hiragana() -> HashMap<String, String> {
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
    fn initialize_katakana() -> HashMap<String, String> {
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
    fn initilize_roomaji(initialize_kana : fn()-> Vec<&'static str>) -> HashMap<String, String> {
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

    /// Returns a vector of strings where each string represents a Japanese syllable
    ///
    /// #Arguments
    ///
    /// * `input` - A str slice that needs to be parsed into Japanese syllables
    /// * `is_eng` - A boolean value that determines whether or not the input is
    ///               an english word
    ///
    /// # Example
    ///
    /// ```
    /// let test_input = "toto";
    /// assert_eq!(to_japanese_syllables(test_input, false), "["to", "to"]);
    /// 
    /// let test input = "grab";
    /// assert_eq!(to_japanese_syllables(test_input, true), "["g", "ra", "b"]);
    /// ```
    ///
    fn to_japanese_syllables(input: &str, is_eng: bool) -> Vec<String> {
        let input = input.to_lowercase();

        // This vector is used to store the syllables for the input string
        let mut syllables = Vec::new();
        // Vowel array for comparison
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        // Possible geminate characters
        let geminates = ['k', 't', 'p', 'g', 'd', 'b', 's', 'z', 'c'];
        // Possible digraphs
        let digraph = ["ky", "sh", "ch", "nq", "hy", "my", "ry", "gy", "by", "py"];

        let mut temp_syllable = "".to_string();
        let mut temp_digraph = "".to_string();
        let mut prev_nasal = false;
        let mut prev_geminate = false;

         if is_eng {
            let mut prevchar = 'a';

            for c in input.chars(){
                temp_syllable.push(c);
                
                if vowels.contains(&c){
                    if !vowels.contains(&prevchar) {
                        syllables.pop();
                        temp_syllable.insert_str(0, &prevchar.to_string());
                        }
                    syllables.push(temp_syllable);
                } else {
                   syllables.push(c.to_string());
                }
                temp_syllable = "".to_string();
                prevchar = c;

            }
        } else {
        for c in input.chars() {
            temp_syllable.push(c);
            temp_digraph.push(c);

            // This check adds non-alphabetic chars to the syllable vector.
            if !c.is_alphabetic() {
                prev_nasal = false;
                syllables.push(c.to_string());
                temp_syllable = "".to_string();
            }

            // This checks to see if it's a digraph
            if digraph.contains(&temp_digraph.as_str()) {
                if prev_geminate {
                    syllables.pop();
                }
                if prev_nasal {
                    syllables.pop();
                }
                temp_syllable = "".to_string();
                temp_syllable.push_str(&temp_digraph);
                prev_geminate = false;
                prev_nasal = false;
            }
            // Japanese syllables can end in final -n, so it needs to be checked.
            if c == 'n' {
                prev_nasal = true;
                syllables.push(c.to_string());
            } else if !vowels.contains(&c) && prev_nasal {
                prev_nasal = false;
                temp_syllable = "".to_string();
                temp_syllable.push(c);
            }
            // This checks the geminate array and sets geminate flag to test for gemination.
            if !vowels.contains(&c) && prev_geminate {
                let temp_char = syllables.pop().unwrap();

                if temp_char == c.to_string() {
                    temp_syllable = "".to_string();
                    syllables.push("G".to_string());
                    temp_syllable.push(c);
                    prev_geminate = false;
                    continue;
                }
            }
            if geminates.contains(&c) {
                prev_geminate = true;
                syllables.push(c.to_string());
            }

            if vowels.contains(&c) {
                if prev_geminate {
                    syllables.pop();
                }

                if prev_nasal {
                    syllables.pop();
                    prev_nasal = false;
                }
                prev_geminate = false;
                syllables.push(temp_syllable);
                temp_syllable = "".to_string();
            }

            if temp_digraph.len() >= 2 {
                temp_digraph = "".to_string();
                temp_digraph.push(c);
            }
        }
        }
        syllables
    }

    ///  Returns a result that gives a string in hiragana on success.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that will be converted to hiragana
    /// * `is_eng` - A boolean value that is set if the input origin is from English
    ///
    ///  # Examples
    ///
    ///  ```
    ///  let input = "kana";
    ///  assert_eq!(to_hiragana(input).unwrap(), "かな");
    ///  ````
    ///
    pub fn to_hiragana(input: &str, is_eng: bool) -> Result<String, String> {
        let mut output = "".to_string();
        let syllables = to_japanese_syllables(input, is_eng);
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
    /// * `is_eng` - A boolean value that is set if the input origin is English
    ///
    ///  # Examples
    ///
    ///  ```
    ///  let input = "kana";
    ///  assert_eq!(to_katakana(input).unwrap(), "カナ");
    ///  ````
    ///
    pub fn to_katakana(input: &str, is_eng: bool) -> Result<String, String> {
        let mut output = "".to_string();

        let syllables = to_japanese_syllables(input, is_eng);

        let mut last_vowel = ' ';
        // After the syllables have been parsed, we can get the kana values for them
        for c in &syllables {
            let mut temp = c.to_string();
            if &last_vowel.to_string() == c {
                // This retrieves the choonpu used for long vowels in katakana.
                temp = "L".to_string();
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

    /// Returns a result that gives a string output of Latin 1 characters
    /// from hiragana input on success
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice in hiragana
    ///
    pub fn to_roomaji_hiragana(input: &str)-> Result<String, String> {
        let mut output = "".to_string();
        
        let mut last_char = ' ';
        // This value is for the small tsu used to mark geminates
        let geminate = '\u{3063}';
        // These are vowels used in the formation of digraphs
        let digraph_vowels = ['\u{3083}', 'a' , '\u{3085}', 'u', '\u{3087}', 'o', 
        '\u{3047}', 'e'];
        // These kana are romanized atypically from other digraph consonants
        let digraph_sibilants = ['\u{3057}', '\u{3058}', '\u{3061}'];
        
        for c in input.chars(){
            if !c.is_alphabetic(){
                output.push(c);
            } else if digraph_vowels.contains(&c) {
                output.pop();

                if !digraph_sibilants.contains(&last_char) {
                    output.push('y');
                }
                let index = digraph_vowels.into_iter().position(|x| x == &c).unwrap();
                output.push(digraph_vowels[index + 1]);
                
            } else {
                let mut temp = c.to_string();
                let result = ROOMAJI_HIRAGANA.get(&temp);
            
                if last_char == geminate {
                    output.pop();
                    let first_char = result.unwrap().chars().next().unwrap();
                    output.push(first_char);
                }
                match result {
                    Some(_) => output.push_str(result.unwrap()),
                    None => return Err("Unable to parse input".to_string()),
                }
                last_char = c;
            }
        }
    
        Ok(output)
        
    }

    /// Returns a result that gives a string output of Latin 1 characters
    /// from katakana input on success
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice in katakana
    ///
    pub fn to_roomaji_katakana(input: &str)-> Result<String, String> {
        let mut output = "".to_string();
        
        let mut last_char = ' ';
        // This value is for the small tsu used to mark geminates
        let geminate = '\u{30C3}';
        // This value is the long vowel marker used in katakana
        let choonpu = "\u{30FC}";
        // These are vowels used in the formation of digraphs
        let digraph_vowels = ['\u{30E3}', 'a' , '\u{30E5}', 'u', '\u{30E7}', 'o', 
        '\u{30A7}', 'e'];
        // These kana are romanized atypically from other digraph consonants
        let digraph_sibilants = ['\u{30B7}', '\u{30B8}', '\u{30C1}'];
        
        for c in input.chars(){
            if !c.is_alphabetic(){
                output.push(c);
            } else if digraph_vowels.contains(&c) {
                output.pop();

                if !digraph_sibilants.contains(&last_char) {
                    output.push('y');
                }
                let index = digraph_vowels.into_iter().position(|x| x == &c).unwrap();
                output.push(digraph_vowels[index + 1]);
                
            } else if c.to_string() == choonpu {
                let last_char = output.chars().last().unwrap();
                output.push(last_char);
                 
            } else {
                let mut temp = c.to_string();
                let mut result = ROOMAJI_KATAKANA.get(&temp);
                if temp == choonpu {
                    result = ROOMAJI_KATAKANA.get(&*last_char.to_string());
                    let last_char = result.unwrap().chars().last().unwrap();
                    output.push(last_char);
                 
                } else {
                if last_char == geminate {
                    output.pop();
                    let first_char = result.unwrap().chars().next().unwrap();
                    output.push(first_char);
                }
                match result {
                    Some(_) => output.push_str(result.unwrap()),
                    None => return Err("Unable to parse input".to_string()),
                }
                }
                last_char = c;
            }
        }

        Ok(output)
    }
}

#[macro_use]
extern crate lazy_static; // 1.0.2

use std::env;
use std::process;
use to_kana::*;
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
    to_hiragana(temp.as_str(), true).expect("to_hiragana from cmu_hiragana failed")
}

fn cmu_katakana(word: &str) -> String {
    let cmu = make_cmu_map();
    let jap = make_jap_map();
    let temp: String = eng_to_jap(word,&cmu,&jap).join("");
    to_katakana(temp.as_str(), true).expect("to_katakana from cmu_katakana failed")
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