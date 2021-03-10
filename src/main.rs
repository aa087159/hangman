use std::io;
use rand::Rng;

fn unique_char(word:String, word_length:usize)->i32 {
    let mut count = 0;
    for letter in 0..word_length {
       let mut appears = false;
       for j in 0..letter{
            if word.chars().nth(j).unwrap() == word.chars().nth(letter).unwrap() {
                appears = true;
                break;
            };
        };
        if !appears{
            count+=1;
        };
    }
    return count;
}

fn main() {
    let words=vec!["cat","doggo","kangaroo"];
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..words.len());
    let current_word = words[random_num];
    let word_length  = current_word.chars().count();
    let unique_words = unique_char(current_word.to_string(),word_length) as f32;
    let mut chances = (unique_words*0.8).floor() as i32;
    let mut user_input:Vec<String> = Vec::new();
    let word_vec:Vec<char>= current_word.chars().collect();
    let mut win = false;

    while chances > 0 && &win==&false{
        println!("*********************");
        //println!("word: {}",&current_word);
        println!("chances: {}",&chances);
        for e in word_vec.iter(){
            if &user_input.contains(&e.to_string())==&true{
                print!("{} ", e);
            }else{
                print!("{} ", "_");
            }
        }
        println!("");
        println!("Please input a letter:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if &guess.is_empty()==&false{
            if &current_word.contains(&guess.trim())==&false {
                chances -=1;
                if chances ==0{
                    println!("game over!");
                }else{
                    println!("wrong guess!");
                }
            }else if current_word.contains(&guess.trim()) && user_input.contains(&guess.trim().to_string())==false{
                &user_input.push(guess.trim().to_string());
                if unique_words as i32 == user_input.len() as i32{
                    win=true;
                    println!("you win!")
                }
            }
        }
    }

}
