use std::io::{self, Write};

fn main() {
    println!("Hi we in Rusty take care of you :)");
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();

    let mut user_name = String::new();
    io::stdin()
        .read_line(&mut user_name)
        .expect("Faild to get your name!");

    println!("Hi {} we are glad to work with you ;)", user_name.trim());
}
