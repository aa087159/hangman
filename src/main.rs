use rand::Rng;
use std::collections::HashSet;
use std::io;

fn main() {
    let words = vec!["cat", "doggo", "kangaroo"];
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..words.len());
    let current_word = words[random_num];
    let split_current_word: Vec<char> = current_word.chars().collect();
    let unique_words = current_word.chars().collect::<HashSet<char>>().len() as f32;
    let mut chances = (unique_words * 0.8).floor() as i32;
    let mut correct_guesses: HashSet<char> = HashSet::new();
    let mut win = false;

    while chances > 0 && !&win {
        println!("*********************");
        println!("word: {}", &current_word);
        println!("chances: {}", &chances);
        for e in split_current_word.iter() {
            if correct_guesses.contains(e) {
                print!("{} ", e);
            } else {
                print!("{} ", "_");
            }
        }

        println!();
        println!("Please input a letter:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if let Some(c) = guess.trim().chars().next() {
            if !current_word.contains(c) || guess.trim().chars().count() > 1 {
                chances -= 1;
                if chances == 0 {
                    println!("game over!");
                } else {
                    println!("wrong guess!");
                }
            } else if current_word.contains(c) && !correct_guesses.contains(&c) {
                correct_guesses.insert(c);
                if unique_words as i32 == correct_guesses.len() as i32 {
                    win = true;
                    println!("you win!")
                }
            }
        }
    }
}
