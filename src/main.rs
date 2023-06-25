use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    // Affiche le message à l'utilisateur
    println!("Bonjour chère utilisateur !");

    // Génère un nombre entier entre 1 et 100
    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    // affichage du nombre
    // println!("Votre nombre: {}", nombre_secret);

    loop {
        // affichage du message
        println!("Veuillez entrer votre nombre entre 1 & 100");

        // allocation en mémoire l'entrée utilisateur
        let mut supposition = String::new();

        // récupération de l'input utilisateur et mutation de la variable supposition
        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entree utilisateur");

        // conversion en type unsigned 32 bits afin d'avoir toujour un nombre positif
        let supposition: u32 = match supposition.trim().parse() {
            // retourne le nombre
            Ok(nombre) => nombre,
            // retoune une erreur si on a pas de nombre
            Err(_) => {
                println!("La valeur que vous avez communiquer est invalide");
                continue;
            },
        };

        // ici j'ai pas trop compris on fait appel a la fonction cmp (compare)
        // on lui passe en paramètre le nombre secret pour la comparaison avec la supposition
        // l'utilisation du borrow ?
        // match ? similaire a if ?
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("C'est exact vous avez gagner !");
                break;
            },
        }
    }
}
