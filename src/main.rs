mod word;

use crate::word::get_word;
use std::num::ParseIntError;

const WORD_LIST: &'static str = include_str!("../word_list.txt");
const WORD_FREQ: &'static str = include_str!("../word_freq.txt");

struct WordData {
    lemma: &'static str,
    freq: u32,
}

fn main() {
    let prefix = "hel";

    let mut words = parse_word_data(WORD_LIST, WORD_FREQ).expect("The word data is invalid");
    words.sort_by(|a, b| a.lemma.cmp(b.lemma));

    let word = get_word(prefix, &words);

    println!("Prefix: {}", prefix);
    match word {
        Some(word) => {
            println!("{}", word);
        }
        None => {
            println!("No word found");
        }
    }
}

fn parse_word_data(
    word_list: &'static str,
    word_freq: &'static str,
) -> Result<Vec<WordData>, ParseIntError> {
    let freqs = word_freq
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(word_list
        .lines()
        .filter(|line| !line.is_empty())
        .zip(freqs)
        .map(|(lemma, freq)| WordData { lemma, freq })
        .collect())
}
