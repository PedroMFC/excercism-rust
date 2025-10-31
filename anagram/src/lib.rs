
use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let mut word_map = HashMap::new();
    for c in word.to_lowercase().chars() {
        let count = word_map.entry(c).or_insert(0);
        *count += 1;
    }
    for possible_anagram in possible_anagrams {
        let mut possible_anagram_map = HashMap::new();
        for c in possible_anagram.to_lowercase().chars() {
            let count = possible_anagram_map.entry(c).or_insert(0);
            *count += 1;
        }
        if word_map == possible_anagram_map && word.to_lowercase() != possible_anagram.to_lowercase() {
            anagrams.insert(*possible_anagram);
        }
    }
    anagrams
}
