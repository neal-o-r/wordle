use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq,PartialOrd)]
struct NonNan(f32);

impl NonNan {
    fn new(val: f32) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_words(fname: &str) -> Vec<String> {
    // read the dictionary into a list
    let fpath = Path::new(fname);
    let f = File::open(&fpath)
                .expect("Couldn't open file");

    let mut dict: Vec<String> = Vec::new();

    for line in BufReader::new(f).lines() {
        dict.push(line.unwrap());
    }
    return dict
}

fn make_a_guess(word: &str, answer: &str) -> String {

    let mut pattern = "".to_string();
    for (w, a) in word.chars().zip(answer.chars()) {
        if w == a {
            pattern.push('G');
        } else if answer.contains(w) {
            pattern.push('Y')
        } else {
            pattern.push('.')
        }
    }
    return pattern
}

fn entropy(word: &str, words: &Vec<String>) -> NonNan {

    let mut tree = HashMap::new();
    let p = 1.0 / words.len() as f32;
    for w in words {
        let pattern = make_a_guess(w, word);
        if tree.contains_key(&pattern) {
            *tree.get_mut(&pattern).unwrap() += p;
        } else {
            tree.insert(pattern, p);
        }
    }

    let mut H = 0.;
    for (_k, v) in tree {
        H += v * v.recip().log2();
    }
    return NonNan::new(H).unwrap();
}

fn filter_words(answer: &str, guess: &str, words: &Vec<String>) -> Vec<String> {
    let p = make_a_guess(guess, answer);
    words.into_iter()
         .filter(|w| (make_a_guess(guess, &w) == p) && (*w != guess))
         .cloned()
         .collect()
}

fn best_guess(words: &Vec<String>) -> String {
    words.into_iter().max_by_key(|w| entropy(w, words)).unwrap().to_string()
}

fn game(answer: &str, words: &Vec<String>, first_word: &str, display: bool) -> i32 {

    let mut possible = words.to_vec();
    let mut guess = first_word.to_string();
    let mut n = 1;

    if display {
        println!("The answer is: {}", answer);
        println!("Guess {}: {}", n, guess);
    }

    while guess != answer {
        possible = filter_words(answer, &guess, &possible);
        guess = best_guess(&possible);
        n += 1;
        if display {
            println!("Guess {}: {}", n, guess);
        }
    }

    return n
}

fn play_all_words(words: &Vec<String>, answers: &Vec<String>) -> Vec<i32> {
    return answers.iter().map(|a| game(a, words, &"aeons", false)).collect()
}


fn main() {
    let words = get_words("../data/words.txt");
    let answers = get_words("../data/answers.txt");

    let results = play_all_words(&words, &answers);
    println!("Words solved in 6: {}", results.iter().filter(|&x| *x <= 6).count());
    println!("Average guesses: {}", results.iter().sum::<i32>() as f32 / results.len() as f32);
}
