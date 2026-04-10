mod lexe;

fn main() {
    println!("Enter expression:"); 

    let mut input = String::new(); 

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("Token: {:?}", lexe::lexer(&mut input));
}
