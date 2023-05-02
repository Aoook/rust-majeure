// programme 0, les bases

use std::io;
use std::io::*;


fn main() {
    // 1. initier et afficher une variable de type string et integer
    let mon_string = "un simple string";
    println!("{}", mon_string);

    let annee = 2023;
    println!("ceci est l'année {}", annee);

    // 2. initier 2 chiffres et les multipliers
    let nombre1 = 10;
    let nombre2 = 20;
    let resultat = nombre1 * nombre2;
    println!("{} * {} = {}", nombre1, nombre2, resultat);

    /* 
    3. initier un nombre. Envoyer le dans une fonction. Demander à l'utilisateur d'entrer un chiffre. 
    Multiplier le premier nombre avec celui entré par l'utilisateur. Sortez de la fonction avec le résultat.
    Affiche le nombre retour
    */

    let nombre1 = 10;
    println!("----");
    println!("initial value: {}", nombre1);
    let return_value: i32 = calculate(nombre1);
    println!("result: {}", return_value);
}

fn calculate(input_number: i32) -> i32{

    println!("Enter a number:");
    let mut input_line = String::new();
    io::stdin() // the rough equivalent of `std::cin`
    .read_line(&mut input_line) // actually read the line
    .expect("Failed to read line"); // which can fail, however
let user_number: i32 = input_line
    .trim() // ignore whitespace around input
    .parse() // convert to integers
    .expect("Input not an integer"); // which, again, can fail


    println!("user value: {}", user_number);
    let resultat = input_number * user_number;
    return resultat;
}