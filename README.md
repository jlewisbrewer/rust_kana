#  Rust Kana Transliterator

This project transliterates English and Japanese words into Japanese characters, and can also take Japanese characters and output the corresponding romanization.

Open source license.

Support for transliterating into and from both hiragana and katakana is included.

## Usage

To run the program, you must input **cargo run [OPTION] [INPUT]** into the command line. For example, to generate the hiragana for "gakkou", you would input:
```
cargo run hiragana gakkou
```
This will output the input text in hiragana:
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/to_kana hiragana gakkou`
がっこう
```

## Options

There are six options you could use.

**hiragana**

This option expects a Japanese input string and will output a hiragana string. While the input does not necessarily need to be a Japanese word, it should still follow Japanese phonological rules.

Example:
```
cargo run hiragana "jibirisu"
じびりす
```
When inputting a Japanese word, this program will accept modern Hepburn romanization practices with some exceptions:
  - Since the program can only accept ASCII input, long vowels will be doubled rather than using a macron. For instance *obāsan* 
  should be input as *obaasan*.
  - Geminates are input by repeating the doubled character in all cases. This differs from Hepburn romanization where まっちゃ is usually romanized as *matcha*. For this program use *maccha*.
  - To resolve the abiguity of final -n (ん) and the initial ny- sound (as in *nya* にゃ), ny- digraphs should be input as nq-. For example:
``` 
cargo run hiragana honya
ほんや
cargo run hiragana honqa
ほにゃ
```

**katakana**

This option accepts a Japanese input string in Latin 1 characters and will output a katakana string. The input requirements are the same for the **hiragana** option.

**cmu_hiragana**

This option will take an English word as an input and transliterate to a hiragana string output. There are two methods used: a rules-based method which parses english words, and a syllable-generator which uses the Carnegie Mellon University phonetic dictionary (http://www.speech.cs.cmu.edu/cgi-bin/cmudict) to first create phonetic syllables, which can then be transliterated into Japanese, hopefully giving a result closer to the actual pronunciation.

Example:
```
cargo run cmu_hiragana "rust"
らすと
```

Note that this option can only take one word at a time as an input.

**cmu_katakana**

This option functions the same as **cmu_hiragana** but will output a katakana string.

Example:
```
cargo run cmu_katakana "rust"
ラスト
```
**roomaji_hiragana**

This option will take a Japanese hiragana string and will output it as Latin 1 unicode. 

Example:
```
cargo run roomaji_hiragana らすと
rasuto
```

**romaji_katakana**

This option will take a Japanese katakana string and will out it as Latin 1 unicode.

Example:
```
cargo run roomaji_katakana ラスト
rasuto
```

