use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_sorted = sort_characters(word);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            sort_characters(candidate) == word_sorted
                && candidate.to_lowercase() != word.to_lowercase()
        })
        .cloned()
        .collect()
}

pub fn sort_characters(s: &str) -> String {
    let mut result = s.to_lowercase().chars().collect::<Vec<char>>();
    result.sort_unstable();
    result.iter().collect::<String>()
}
