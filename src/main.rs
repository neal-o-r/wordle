use std::path::Path;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

// Struct to store what
// we know about the answer
// this is obviously a bit redundant
// (all info is in here twice) but it's
// easier to work with like this
#[derive(Debug)]
struct Info {
    in_word: String,
    not_word: String,
    in_pos: Vec<(usize, char)>,
    not_pos: Vec<(usize, char)>,
}


fn get_dict(fname: &str) -> Vec<String> {
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


fn is_valid(word: &str, info: &Info) -> bool {
    //apply all the checks to a word
    if !info.in_word.chars().all(|x| word.contains(x)) {
        // does word contain all letters it should
        return false;
    }
    if info.not_word.chars().any(|x| word.contains(x)) {
        // does it contain any letters it shouldn't
        return false
    }
    for (i, l) in &info.in_pos {
        //are all the letters where they should be
        if word.chars().nth(*i).unwrap() != *l {
           return false
        }
    }
    for (i, l) in &info.not_pos {
        // are there letters where there shouldn't
        if word.chars().nth(*i).unwrap() == *l {
            return false
        }
    }
    return true
}


fn filter_words(words: &Vec<String>, info: &Info) -> Vec<String> {
    // get all words that match info
    words.into_iter().filter(|&x| is_valid(&x, info)).cloned().collect()
}

fn overlap(w1: &str, w2: &str) -> u32 {
    // how many (unique) letters do 2 words share
    let set: HashSet<char> = w1.chars().collect();
    w2.chars().filter(|c| set.contains(&c)).count() as u32
}

fn total_overlap(w1: &str, words: &Vec<String>) -> u32 {
    // get the overlap between a word and all others
    words.into_iter().map(|x| overlap(w1, x)).sum()
}

fn best_guess(words: &Vec<String>) -> String {
    // the best word is the word with most letters in common with
    // most words in the list.
    words.into_iter().max_by_key(|x| total_overlap(x, words)).unwrap().to_string()
}

fn make_guess(guess: &str, answer: &str, info: &Info) -> Info {
    // combine info with new info from our guess
    let in_word: String = guess.chars().filter(|x| answer.contains(&x.to_string())).collect();
    let not_word: String = guess.chars().filter(|x| !answer.contains(&x.to_string())).collect();
    let mut pos = info.in_pos.to_vec();
    let mut not_pos = info.not_pos.to_vec();
    for (i, l) in guess.chars().enumerate() {
        if l == answer.chars().nth(i).unwrap() {
            pos.push((i, l));
        } else {
            not_pos.push((i, l));
        }
    }
    return Info {in_word : in_word + &info.in_word,
                 not_word: not_word + &info.not_word,
                 in_pos  : pos,
                 not_pos : not_pos}
}


fn play_a_game(dict: &Vec<String>) -> bool {

    let mut words = dict.to_vec();

    let answer = dict.choose(&mut rand::thread_rng()).unwrap();
    println!("The chosen word is: {}", answer);

    let mut info = Info {in_word: "".to_string(), not_word:"".to_string(), in_pos:vec![], not_pos:vec![]};

    let mut guess = "aeons".to_string();
    //always start with this word

   // try six times. update our info, based on the guess
   // filter out the words that don't agree with the info
   // make the best guess
    for c in 0..6 {
        
        println!("{}, {}", c, guess);
        if &guess == answer {
            return true
        }

        info = make_guess(&guess, &answer, &info);
        words = filter_words(&words, &info);

        guess = best_guess(&words);
    }

    return false

}


fn main() {
    let dict = get_dict("../data/words5.txt");
    play_a_game(&dict);
}
