use std::collections::HashSet;

fn main() {

    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Failed");
    let char_set: HashSet<char> = input_str.chars().collect();


    if ( char_set.len() % 2 ) == 0{
        println!("CHAT WITH HER!");

    }else{
        println!("IGNORE HIM!");
    }

}