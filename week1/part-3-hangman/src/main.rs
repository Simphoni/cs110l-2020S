// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut remaining_guesses = NUM_INCORRECT_GUESSES;
    let mut hit: Vec<bool> = Vec::new();
    let mut history = String::new();
    let mut vis: HashSet<char> = HashSet::new();
    hit.resize(secret_word_chars.len(), false);
    let mut finish = false;
    while remaining_guesses > 0 && !finish {
        print!("The word so far is ");
        for i in 0..secret_word_chars.len() {
            if hit[i] == true {
                print!("{}", secret_word_chars[i]);
            } else {
                print!("-")
            }
        }
        print!("\n");
        print!("You have guessed the following letters: {}\n", history);
        print!("You have {} guesses left\n", remaining_guesses);
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        print!("\n");
        let chr: Vec<char> = guess.chars().collect();
        let c = chr[0];
        if vis.insert(c) {
            finish = true;
            history = history + &String::from(c);
            let mut correct = false;
            for i in 0..secret_word_chars.len() {
                if secret_word_chars[i] == c {
                    hit[i] = true;
                    correct = true;
                }
                finish &= hit[i];
            }
            if correct == false {
                print!("Sorry, that letter is not in the word\n");
                remaining_guesses -= 1;
            }
        } else {
            print!("Character previously requested, chance is wasted.\n");
            remaining_guesses -= 1;
        }
    }
    if finish {
        print!(
            "Congratulations you guessed the secret word: {}!\n",
            secret_word
        );
    } else {
        print!("Sorry, you ran out of guesses!\n");
    }
}
