use std::collections::HashSet;


fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {

    let input = get_input();
    // {b, a, b, a}
    let l: &[char] = &['}','{',' ',','];
    let new_input = input.trim();

    let my_set: HashSet<&str>= new_input.split(',').into_iter().map(|x| x.trim_matches(l)).filter(|y| (! y.is_empty())).rev().collect();

    println!("{}",my_set.len())
}