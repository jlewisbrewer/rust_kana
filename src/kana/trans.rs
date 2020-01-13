use KATAKANA;
use HIRAGANA;
use ROOMAJI_HIRAGANA;
use ROOMAJI_KATAKANA;

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
pub fn to_japanese_syllables(input: &str, is_eng: bool) -> Vec<String> {
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
        if &last_vowel.to_string() == c && last_vowel.is_alphabetic() {
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

/// Tests
/// 

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