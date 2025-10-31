
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

// Solution from community
// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let word_lower = word.to_lowercase();
//     let word_sorted = get_sorted(&word_lower);
//     possible_anagrams
//         .iter()
//         .filter(|candidate| {
//             let candidate_lower = candidate.to_lowercase();
//             candidate_lower.len() == word_lower.len()
//                 && candidate_lower != word_lower
//                 && get_sorted(&candidate_lower) == word_sorted
//         }).copied()
//         .collect()
// }
// fn get_sorted(word: &str) -> Vec<char> {
//     let mut word_sorted: Vec<char> = word.chars().collect();
//     word_sorted.sort_unstable();
//     word_sorted
// }