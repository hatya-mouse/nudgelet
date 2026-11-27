use crate::WordData;

pub(super) fn get_word(prefix: &str, words: &[WordData]) -> Option<&'static str> {
    let start_index = words.partition_point(|word| word.lemma < &prefix);
    let end_index =
        start_index + words[start_index..].partition_point(|word| word.lemma.starts_with(prefix));
    let candidates = &words[start_index..end_index];

    candidates
        .iter()
        .max_by_key(|word| word.freq)
        .map(|word| word.lemma)
}
