use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_low_cased = word.to_lowercase();
    let mut sorted_low_cased_word: Vec<char> = word_low_cased.chars().collect(); //  into a collection.
    sorted_low_cased_word.sort_unstable();

    let mut found: HashSet<&'a str> = HashSet::new();

    for candidate_word in possible_anagrams {
        if word_low_cased.len() == candidate_word.len() {
            let candidate_word_low_cased = candidate_word.to_lowercase();
            let mut candidate_word_low_cased_sorted: Vec<char> =
                candidate_word_low_cased.chars().collect();
            candidate_word_low_cased_sorted.sort_unstable();

            // becasue it's insensitive
            if word_low_cased != candidate_word_low_cased {
                continue;
            }

            if sorted_low_cased_word == candidate_word_low_cased_sorted {
                found.insert(candidate_word);
            }
        }
    }
    found

    /*
      Troubles:
        - Misunderstanding
          - an asnwer same as input is not included.
            according to "The solution cannot contain the input word."
          - should select the sublist, not first one word
      TODO:
        - Don't rush, and take time to understand fully. check again my understanding
    */
}
