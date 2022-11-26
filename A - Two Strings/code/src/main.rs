

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {

    let n = get_input().trim().parse::<i32>().unwrap();

    let mut results = String::new();

    for i in 0..n{ 
        let mut string1 = get_input();


        let mut string2 = get_input();
        
        let mut match_founded = false;

        for c in string1.trim().chars(){

            if string2.contains(c){
                match_founded = true;
                break;
            }
        }
        if match_founded{
            results.push_str("YES\n");
        }else{
            results.push_str("NO\n");
        }

    }
    println!("{}",results.trim());

}