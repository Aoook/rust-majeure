// programme 2, création mini-programme

//use std::io;
use rand::Rng;
use read_input::prelude::*;
use std::process;

fn main() {

    let loop_jeux: i32 = 0;
    let mut difficulte: i32 = 1;
    /* MAIN */
    while loop_jeux == 0 {
        let choix_menu: i32 = menu_principal();

       // A COMPLETER
    }

    /* MENU */
    fn menu_principal() -> i32{
        // A COMPLETER
    }

    /* CHOIX DIFFICULTE */
    fn choix_difficulte() -> i32{
        // A COMPLETER
    }

    /* LOOP DU JEUX */
    fn lancer_jeux(difficulte: i32){

        // création chiffre aléatoire
        let mut numero_max: i32 = 0;
        if difficulte == 1{
            numero_max = 10;
        } else if difficulte == 2{
            numero_max = 100;
        } else if difficulte == 3{
            numero_max = 1000;
        } else {
            println!("erreur attribution");
        }

        let number_rand: i32 = rand::thread_rng().gen_range(0..{numero_max});
        

        // loop trouver le bon numero
        let trouver: i32 = 0;
        println!("Donnez un chiffre");
        let mut choix_utilisateur: i32 = input::<i32>().get();

        while trouver == 0{

            if choix_utilisateur == number_rand {
               // A COMPLETER
            } else if choix_utilisateur < number_rand {
               // A COMPLETER
            } else if choix_utilisateur > number_rand {
                // A COMPLETER
            } else {
                println!("erreur lors de la validation du bon chiffre");
            }

            choix_utilisateur = input::<i32>().get();

        }



        println!("----- Debut du jeux -----\n\n");
        println!("Donnez un chiffre");

    }

    /* QUITTER LE JEUX */
    fn quitter_programme(){
        process::exit(1);
    }

}
