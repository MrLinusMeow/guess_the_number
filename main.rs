fn main() {
    let mut i = true;
    let number =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos()
        % 2;
    println!("Guess the number!");
    while i {
        let mut guess = String::new();

        println!("Please input your guess:");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess_number = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1B[91mNot a number :(\x1B[0m");
                continue;
            },
        };
        println!("You guessed: {};", guess_number);

        if guess_number == number {
            println!("\x1B[93m[*]\tYou have guessed it right.\x1B[0m");
            i = false;
            guess.clear();
            continue;
        }else if guess_number < number {
            println!("\x1B[91m[+]\thiger\x1B[0m");
            guess.clear();
            continue;
        }else if guess_number > number {
            println!("\x1B[91m[-]\tlesser\x1B[0m");
            guess.clear();
            continue;
        }
    }
}
