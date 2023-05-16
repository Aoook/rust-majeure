use std::io;

fn main() {
    println!("Entrez un nombre :");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Erreur de lecture");

    let number: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Veuillez entrer un nombre valide");
            return;
        }
    };

    let result = fibonacci(number);

    println!("Le {} ème nombre de la suite de Fibonacci est : {}", number, result);
}

fn fibonacci(n: i32) -> i32 {
    // A COMPLETER
}
