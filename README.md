#  Rust Kana Transliterator

This project transliterates English and Japanese words into Japanese characters.

Open source license.

Support for transliterating into both hiragana and katakana is included.

## Usage


There are two methods used: a rules-based method which parses english words, and a syllable-generator which uses the Carnegie Mellon University phonetic dictionary (http://www.speech.cs.cmu.edu/cgi-bin/cmudict) to first create phonetic syllables, which can then be transliterated into Japanese, hopefully giving a result closer to the actual pronunciation.
