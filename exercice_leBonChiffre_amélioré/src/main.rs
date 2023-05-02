// programme 3, amélioration

/*
ajout de 2 fonctions:
- sauvegarde automatique du score dans un fichier .txt
- ajout d'un choix dans le MENU pour visualiser le dernier score.
*/

//use std::io;
use rand::Rng;
use read_input::prelude::*;
use std::process;
use std::fs;

fn main() {

    let loop_jeux: i32 = 0;
    let mut difficulte: i32 = 1;
    /* MAIN */
    while loop_jeux == 0 {
        let choix_menu: i32 = menu_principal();

        if choix_menu == 1{
            difficulte = choix_difficulte();
        } else if choix_menu == 2{
            lancer_jeux(difficulte);
        } else if choix_menu == 3{
            quitter_programme();
        } else if choix_menu == 4{
            visualisation_score();
        } else {
            println!("erreur choix du menu");
        }
    }

    /* MENU */
    fn menu_principal() -> i32{
        println!("1, choix de la difficulte");
        println!("2, lancer le jeux");
        println!("3, quitter le programme");
        println!("4, visualiser les scores");

        let mut choix_utilisateur: i32 = input::<i32>().get();
        while choix_utilisateur != 1 && choix_utilisateur != 2 && choix_utilisateur != 3 && choix_utilisateur != 4 {
            println!("Merci d'entrer un nombre valide entre 1 et 4.");
            choix_utilisateur = input::<i32>().get();
        }

        return choix_utilisateur;
    }

    /* CHOIX DIFFICULTE */
    fn choix_difficulte() -> i32{
        println!("1, entre 1 et 10");
        println!("2, entre 1 et 100");
        println!("3, entre 1 et 1000");
        
        let mut choix_utilisateur: i32 = input::<i32>().get();
        while choix_utilisateur != 1 && choix_utilisateur != 2 && choix_utilisateur != 3 {
            println!("Merci d'entrer un nombre valide entre 1 et 3.");
            choix_utilisateur = input::<i32>().get();
        }

        return choix_utilisateur
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
        let mut nombre_de_loop = 1;
        println!("Donnez un chiffre");
        let mut choix_utilisateur: i32 = input::<i32>().get();

        while trouver == 0{

            if choix_utilisateur == number_rand {
                println!("trouve, le chiffre était bien: {number_rand}");

                // rajout du nombre d'essai dans le fichier local
                let fichier_input = format!("la dernière partie a été réussi avec {} essais", nombre_de_loop);
                fs::write("./derniere_essai.txt", fichier_input).expect("Unable to write file");


                println!("terminer avec {} essai", nombre_de_loop);

                return ;
            } else if choix_utilisateur < number_rand {
                println!("le chiffre est plus grand");
                nombre_de_loop = nombre_de_loop + 1;
            } else if choix_utilisateur > number_rand {
                println!("le chiffre est plus petit");
                nombre_de_loop = nombre_de_loop + 1;
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

    /* VISUALISER LE SCORE */
    fn visualisation_score(){
        println!("--- visualisation des scores ---\n\n"); // init

        // lecture et affichage du fichier
        let fichier_output = fs::read_to_string("./derniere_essai.txt").expect("Unable to read file");
        println!("{}", fichier_output);

        println!("-------------\n\n"); // end
    }
}
