use std::collections::HashMap;

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}


fn main() {

    let n = get_input().trim().parse::<i32>().unwrap();
    let mut reaults = String::new();
    let mut db:HashMap<String, i32> = HashMap::new();

    for _i in 0..n{
        let name = String::from(get_input().trim());

        match db.get(&name) {
            Some(&number) => {
                reaults.push_str(&name);
                reaults.push_str(&number.to_string());

                reaults.push('\n');
                *db.entry(name).or_insert(0) += 1;

            },
            _ => {
                db.insert(name, 1);
                reaults.push_str("OK\n");
            },
        }

    }
    print!("{}",reaults)
}