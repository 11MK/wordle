use rand::Rng;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Wordle {
    answer: String,
    guesses: Vec<String>,
    guess_letters: Vec<Letter>,
    guess_string: String,
    victory: bool,
    dictionary: Vec<String>,
}

#[derive(Debug)]
pub enum Character {
    Wrong,
    Exact,
    Near,
}

#[derive(Debug)]
pub struct Letter {
    pub ascii: char,
    pub state: Character,
}
impl Letter {
    fn to_string(&self) -> String {
        self.ascii.to_string()
    }
}

impl Wordle {
    // Function to initialize the game
    pub fn initialize() -> Wordle {
        Wordle {
            dictionary: Self::get_dictionary("words.txt"),
            answer: Self::set_game_answer("words.txt"),
            guesses: Vec::new(),
            victory: false,
            guess_letters: Vec::new(),
            guess_string: "".to_string(),
        }
    }

    fn get_dictionary(filename: &str) -> Vec<String> {
        let all_words: Vec<String> = read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect(); // gather them together into a vector
        all_words
    }

    // Private function to select random word from words file
    fn set_game_answer(filename: &str) -> String {
        let mut rng = rand::thread_rng();
        let mut all_words: Vec<String> = read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect(); // gather them together into a vector
        let random_index: usize = rng.gen_range(0..all_words.len());
        all_words.remove(random_index)
    }

    // Removes last character from word list
    pub fn delete_last_char(&mut self) {
        let word_length = self.guess_letters.len();
        if word_length == 0 {
            return;
        }
        self.guess_letters.remove(word_length - 1);
    }

    // Removes last character from word list
    pub fn update_guess(&mut self, letter: Letter) {
        if self.guess_letters.len() == 5 {
            return;
        }
        self.guess_letters.push(letter);
    }

    pub fn validate_guess(mut self) -> bool {
        let guess: String = self
            .guess_letters
            .into_iter()
            .map(|letter| letter.to_string().to_uppercase())
            .collect();
        if self.dictionary.contains(&guess) {
            self.guess_string = guess;
            return true;
        }
        false
    }

    // Algorithm to evaluate guess against answer
    pub fn submit_guess(&mut self) {
        if self.guess_string == self.answer {
            self.victory = true;
        }
        let mut used: Vec<usize> = vec![0, 0, 0, 0, 0];
        let answer_chars: Vec<char> = self.answer.chars().collect();
        for (index, mut letter) in self.guess_letters.iter_mut().enumerate() {
            if answer_chars[index] == letter.ascii {
                letter.state = Character::Exact;
            }
            else if !self.answer.contains(letter.ascii) {
                letter.state = Character::Wrong;
            }
            else {
                for (n, ch) in answer_chars.iter().enumerate() {
                    let ascii_val = letter.ascii as usize;
                    if ch == &letter.ascii && used[n] != ascii_val {
                        used[n] = ascii_val;
                        letter.state = Character::Near;
                        return;
                    }
                }
                letter.state = Character::Wrong;
            }
        }
    }
}

// TODO: (enter) Function that compares guess to answer
//  - yellow box == correct letter in wrong place (! 1:1 grey box for each correct letter)
//  - green box == correct letter in correct place
//  - grey box == wrong letter; letter not in answer

// TODO: (char) Function to update current box with letter
