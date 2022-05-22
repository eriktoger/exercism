use std::collections::HashSet;

fn sort_string(input: &str) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    input_chars.sort();
    String::from_iter(input_chars)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut answer = HashSet::new();
    let word_lowercase = word.to_lowercase();
    let sorted_word = sort_string(&word_lowercase);
    for anagram in possible_anagrams {
        let anagram_lowercase = anagram.to_lowercase();
        let sorted_anagram = sort_string(&anagram_lowercase);
        let is_anagram_different_from_word =
            sorted_anagram == sorted_word && anagram_lowercase != word_lowercase;

        if is_anagram_different_from_word {
            answer.insert(*anagram);
        }
    }
    return answer;
}
