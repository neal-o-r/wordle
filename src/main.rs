use std::path::Path;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


#[derive(Debug, Clone)]
struct Info {
    in_word: String,
    not_word: String,
    in_pos: Vec<(usize, char)>,
    not_pos: Vec<(usize, char)>,
}


fn get_dict(fname: &str) -> Vec<String> {
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
    if !info.in_word.chars().all(|x| word.contains(x)) {
        return false;
    }
    if info.not_word.chars().any(|x| word.contains(x)) {
        return false
    }
    for (i, l) in &info.in_pos {
       if word.chars().nth(*i).unwrap() != *l {
           return false
       }
    }
    for (i, l) in &info.not_pos {
        if word.chars().nth(*i).unwrap() == *l {
            return false
        }
    }
    return true
}


fn filter_words(words: &Vec<String>, info: &Info) -> Vec<String> {
    words.into_iter().filter(|&x| is_valid(&x, info)).cloned().collect()
}

fn overlap(w1: &str, w2: &str) -> u32 {
    let set: HashSet<char> = w1.chars().collect();
    w2.chars().filter(|c| set.contains(&c)).count() as u32
}

fn total_overlap(w1: &str, words: &Vec<String>) -> u32 {
    words.into_iter().map(|x| overlap(w1, x)).sum()
}

fn best_guess(words: &Vec<String>) -> String {
    words.into_iter().max_by_key(|x| total_overlap(x, words)).unwrap().to_string()
}

fn make_guess(guess: &str, answer: &str, info: &Info) -> Info {

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

    let mut c = 0;
    let mut info = Info {in_word: "".to_string(), not_word:"".to_string(), in_pos:vec![], not_pos:vec![]};
    let mut guess = "".to_string();

    while c < 6 && &guess != answer {
        if c == 0 {
            guess = "raise".to_string()
        } else {
            guess = best_guess(&words);
        }
        println!("{}, {}", c, guess);
        if &guess == answer {
            return true
        }

        info = make_guess(&guess, &answer, &info);
        words = filter_words(&words, &info);
        c += 1;
    }

    return false

}


fn main() {
    let dict = get_dict("../data/words5.txt");
    play_a_game(&dict);
}
