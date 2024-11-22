use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    
    let quotes = vec![
        "Life is what happens when you're busy making other plans.",
        "The best way to predict the future is to invent it.",
        "To be yourself in a world that is constantly trying to make you something else is the greatest accomplishment."
    ];
    let quote = quotes.choose(&mut rand::thread_rng()).unwrap();
    println!("{}", quote);

}
