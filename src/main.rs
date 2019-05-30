extern crate rand;

use rand::Rng;
use std::io::{self, BufRead};

static WORDS: &'static str = include_str!("./top-10000-words.txt");

fn main() {
	let stdin = io::stdin();
	let mut progress: u8 = 0;
	let word = get_random_word();
	let mut display = "_".repeat(word.len());
	let mut letters: [bool; 26] = [false; 26];

	println!("Type a letter to guess that letter.");
	println!("You win when you guess the entire word.");
	println!("You lose when the hangman is completed.");
	println!("Type quit to quit.");

	loop {

		display_hangman(progress);
		println!("Word: {}", display);
		display_letters(letters);

		let line = stdin.lock().lines().next().unwrap().unwrap();

		if line == "quit" {
			break;
		}

		clear_console();

		if line.len() != 1 {
			println!("{} is more than one letter.", line);
			continue;
		}

		// Get the first character of the input string, which is the only character
		if let Some(ch) = line.chars().next() {
			let ch = ch.to_ascii_uppercase();
			if !ch.is_alphabetic() {
				println!("{} isn't a letter.", ch);
				continue;
			}
			// Get the distance from A
			let letter = ch as u32 - 'A' as u32;

			if letters[letter as usize] {
				println!("The letter {} has already been guessed.", ch);
				continue;
			}
			// Mark the letter as used
			letters[letter as usize] = true;

			if word.contains(ch) {
				display = display
	                .chars()
	                .zip(word.chars())
	                .map(|(current, correct)| if correct == ch {
	                    correct
	                } else {
	                    current
	                })
					.collect();
					println!("Yep! There's a {}!", ch);
			} else {
				println!("Nope! No {}.", ch);
				progress += 1;
			}

			if display == word {
				println!("You Win! The word was {}!", word);
				break;
			}

			if progress >= 7 {
				// Display lose message
				println!("You is dead.");
				println!("The word was {}, by the way.", word);
				println!("Your guess was {}.", display);
				break;
			}
		} else {
			println!("{} isn't a letter.", line);
			continue;
		}		
	}
}

fn display_letters(letters: [bool; 26]) {
	print!("Used: [");
	for (idx, letter) in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
		if letters[idx] {
			print!("{}", letter);
		} else {
			print!(" ");
		}
	}
	println!("]");
}

fn display_hangman(progress: u8) {
	/*

	 o 
	/|\
	 | 
	/ \

	*/
	println!(" {} ", if progress >= 1 {'o'} else {' '});
	print!("{}", if progress >= 3 {'/'} else {' '});
	print!("{}", if progress >= 2 {'|'} else {' '});
	println!("{}", if progress >= 4 {'\\'} else {' '});
	println!(" {} ", if progress >= 5 {'|'} else {' '});
	print!("{} ", if progress >= 6 {'/'} else {' '});
	println!("{}", if progress >= 7 {'\\'} else {' '});
}

fn get_random_word() -> String {
	let num_lines = 9897; // TODO: count automatically

	let mut rng = rand::thread_rng();
	let line_num = rng.gen_range(0, num_lines);

	let word = WORDS.lines().nth(line_num).unwrap().to_ascii_uppercase();

	//println!("Line number is {}.", line_num + 1);

	return String::from(word);
}

fn clear_console() {
	print!("{}[2J", 27 as char);
}
