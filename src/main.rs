mod lexe;
mod parse;

fn main() {
    println!("Enter expression:"); 

    let mut input = String::new(); 

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let mut result: i32 = 0;

    result = parse::eval(&parse::parse(&lexe::lexer(&mut input)));

    println!("Resultado {result}");
}
